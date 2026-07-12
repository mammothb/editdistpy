use crate::common::{resolve_inputs, trim_affix};

#[cfg_attr(not(feature = "python"), allow(dead_code))]
pub fn distance(string_1: Option<&str>, string_2: Option<&str>, max_distance: i64) -> i32 {
    let (s1, s2) = match resolve_inputs(string_1, string_2, max_distance) {
        Ok(pair) => pair,
        Err(distance) => return distance,
    };

    let mut v1: Vec<_> = s1.chars().collect();
    let mut v2: Vec<_> = s2.chars().collect();
    if v1.len() > v2.len() {
        std::mem::swap(&mut v1, &mut v2);
    }
    if v2.len() as i64 - v1.len() as i64 > max_distance {
        return -1;
    }

    let (t1, t2) = match trim_affix(&v1, &v2, max_distance) {
        Ok(pair) => pair,
        Err(distance) => return distance,
    };

    if max_distance < 4 {
        return fujimoto2018(t2, t1, max_distance);
    }
    if max_distance < t2.len() as i64 {
        internal_distance_max(t1, t2, max_distance)
    } else {
        internal_distance(t1, t2)
    }
}

/// All possible edit sequences for max_distance <= 3.
/// 01=DELETE from v1, 10=INSERT to v1, 11=REPLACE.
/// Indexed by `(max^2 + max) / 2 + len_diff - 1`, null-terminated.
static OPS_MATRIX: [[u8; 8]; 9] = [
    // max_distance=1
    [0x03, 0, 0, 0, 0, 0, 0, 0], // len_diff=0
    [0x01, 0, 0, 0, 0, 0, 0, 0], // len_diff=1
    // max_distance=2
    [0x0f, 0x09, 0x06, 0, 0, 0, 0, 0], // len_diff=0
    [0x0d, 0x07, 0, 0, 0, 0, 0, 0],    // len_diff=1
    [0x05, 0, 0, 0, 0, 0, 0, 0],       // len_diff=2
    // max_distance=3
    [0x3f, 0x27, 0x2d, 0x39, 0x36, 0x1e, 0x1b, 0], // len_diff=0
    [0x3d, 0x37, 0x1f, 0x25, 0x19, 0x16, 0, 0],    // len_diff=1
    [0x35, 0x1d, 0x17, 0, 0, 0, 0, 0],             // len_diff=2
    [0x15, 0, 0, 0, 0, 0, 0, 0],                   // len_diff=3
];

fn fujimoto2018(v1: &[char], v2: &[char], max_distance: i64) -> i32 {
    let len1 = v1.len();
    let len2 = v2.len();
    let len_diff = len1 - len2;

    let ops_idx = (max_distance + max_distance * max_distance) as usize / 2 + len_diff - 1;
    let ops_list = OPS_MATRIX[ops_idx];
    let mut cost = max_distance + 1;

    for &ops_byte in ops_list.iter().take_while(|&&b| b != 0) {
        let mut ops = ops_byte;
        let mut v1_pos = 0;
        let mut v2_pos = 0;
        let mut curr_cost = 0;

        while v1_pos < len1 && v2_pos < len2 {
            if v1[v1_pos] != v2[v2_pos] {
                curr_cost += 1;
                if ops == 0 {
                    break; // no ops remaining
                }
                if ops & 1 != 0 {
                    v1_pos += 1; // delete from v1
                }
                if ops & 2 != 0 {
                    v2_pos += 1; // insert into v1 (delete from v2)
                }
                ops >>= 2; // next operation
            } else {
                v1_pos += 1;
                v2_pos += 1;
            }
        }
        curr_cost += (len1 - v1_pos + len2 - v2_pos) as i64;
        if curr_cost < cost {
            cost = curr_cost;
        }
    }

    if cost <= max_distance {
        cost as i32
    } else {
        -1
    }
}

fn internal_distance(v1: &[char], v2: &[char]) -> i32 {
    let len2 = v2.len();
    let mut char_1_costs: Vec<_> = (0..len2).map(|j| j as i32 + 1).collect();
    let mut curr_cost = 0;

    for (i, &c1) in v1.iter().enumerate() {
        let mut above_char_cost = i as i32;
        let mut left_char_cost = i as i32;

        for j in 0..len2 {
            curr_cost = left_char_cost;
            left_char_cost = char_1_costs[j];

            if c1 != v2[j] {
                if above_char_cost < curr_cost {
                    curr_cost = above_char_cost;
                }
                if left_char_cost < curr_cost {
                    curr_cost = left_char_cost;
                }
                curr_cost += 1;
            }
            above_char_cost = curr_cost;
            char_1_costs[j] = curr_cost;
        }
    }

    curr_cost
}

fn internal_distance_max(v1: &[char], v2: &[char], max_distance: i64) -> i32 {
    let len2 = v2.len();
    let max = max_distance as i32;
    let mut char_1_costs: Vec<_> = (0..len2)
        .map(|j| {
            let val = j as i32 + 1;
            if val < max + 1 { val } else { max + 1 }
        })
        .collect();

    let len_diff = len2 - v1.len();
    let j_start_offset = max - len_diff as i32;
    let mut j_start = 0;
    let mut j_end = max as usize;
    let mut curr_cost = 0;

    for (i, &c1) in v1.iter().enumerate() {
        let mut above_char_cost = i as i32;
        let mut left_char_cost = i as i32;

        if i as i32 > j_start_offset {
            j_start += 1;
        }
        if j_end < len2 {
            j_end += 1;
        }

        for j in j_start..j_end {
            curr_cost = left_char_cost;
            left_char_cost = char_1_costs[j];

            if c1 != v2[j] {
                if above_char_cost < curr_cost {
                    curr_cost = above_char_cost;
                }
                if left_char_cost < curr_cost {
                    curr_cost = left_char_cost;
                }
                curr_cost += 1;
            }
            above_char_cost = curr_cost;
            char_1_costs[j] = curr_cost;
        }
        if char_1_costs[i + len_diff] > max {
            return -1;
        }
    }

    if curr_cost <= max { curr_cost } else { -1 }
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rstest::rstest;

    /// Reference Levenshtein
    fn reference_lev(s1: &str, s2: &str, max_distance: i64) -> i32 {
        let v1: Vec<char> = s1.chars().collect();
        let v2: Vec<char> = s2.chars().collect();
        let len1 = v1.len();
        let len2 = v2.len();

        let mut d = vec![vec![0i32; len2 + 1]; len1 + 1];
        for i in 0..=len1 {
            d[i][0] = i as i32;
        }
        for j in 0..=len2 {
            d[0][j] = j as i32;
        }
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if v1[i - 1] == v2[j - 1] { 0 } else { 1 };
                d[i][j] = (d[i - 1][j] + 1)
                    .min(d[i][j - 1] + 1)
                    .min(d[i - 1][j - 1] + cost);
            }
        }
        let dist = d[len1][len2];
        if dist <= max_distance as i32 {
            dist
        } else {
            -1
        }
    }

    fn permuted_strings() -> Vec<String> {
        let alphabet = ['a', 'b', 'c', 'd'];
        let mut result = vec![String::new()];
        for k in 1..=alphabet.len() {
            for combo in alphabet.iter().combinations(k) {
                for perm in combo.iter().permutations(k) {
                    result.push(perm.into_iter().map(|c| **c).collect());
                }
            }
        }
        result
    }

    #[rstest]
    #[case(0)]
    #[case(1)] // fujimoto2018
    #[case(2)] // fujimoto2018
    #[case(3)] // fujimoto2018
    #[case(4)] // internal_distance
    #[case(i32::MAX as i64)]
    fn test_against_reference(#[case] max_distance: i64) {
        let strings = permuted_strings();
        for s1 in &strings {
            for s2 in &strings {
                let expected = reference_lev(s1, s2, max_distance);
                let actual = distance(Some(s1), Some(s2), max_distance);
                assert_eq!(
                    actual, expected,
                    "lev({:?}, {:?}, {}) = {}, expected {}",
                    s1, s2, max_distance, actual, expected
                );
            }
        }
    }

    #[rstest]
    #[case("", "", 10, 0)]
    #[case("abc", "", 10, 3)]
    #[case("", "abc", 10, 3)]
    #[case("abc", "abc", 10, 0)]
    #[case("", "abc", 2, -1)]
    fn test_basic(
        #[case] a: &str,
        #[case] b: &str,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        assert_eq!(distance(Some(a), Some(b), max_distance), expected);
    }

    #[rstest]
    #[case("kitten", "sitting", 10, 3)]
    #[case("flintstone", "hanson", 10, 6)]
    #[case("flintstone", "hanson", 2, -1)]
    #[case("saturday", "sunday", 10, 3)]
    #[case("abcdef", "azced", 10, 3)]
    fn test_known(
        #[case] a: &str,
        #[case] b: &str,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        assert_eq!(distance(Some(a), Some(b), max_distance), expected);
    }

    #[test]
    fn test_max_distance_cutoff() {
        assert_eq!(distance(Some("abc"), Some("abc"), 0), 0);
        assert_eq!(distance(Some("abc"), Some("abd"), 0), -1);
        assert_eq!(distance(Some("abc"), Some("abd"), 1), 1);
        assert_eq!(distance(Some("abc"), Some("xyz"), 2), -1);
    }

    // ==============================
    // internal_distance_max coverage
    // ==============================

    #[rstest]
    // all chars differ, trimmed len=10 > max=5 -> -1
    #[case("aa1111111111zz", "aa2222222222zz", 5, -1)]
    // end chars differ, trimmed len=10 > max=5 -> 2
    #[case("aa1111111111zz", "aa2111111112zz", 5, 2)]
    fn test_internal_distance_max(
        #[case] a: &str,
        #[case] b: &str,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        assert_eq!(distance(Some(a), Some(b), max_distance), expected);
    }

    // ==============================
    // None / resolve_inputs coverage
    // ==============================

    #[rstest]
    #[case(None, None, 10, 0)]
    #[case(None, Some("abc"), 10, 3)]
    #[case(Some("abc"), None, 10, 3)]
    #[case(None, Some("abc"), 2, -1)]
    fn test_none(
        #[case] a: Option<&str>,
        #[case] b: Option<&str>,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        assert_eq!(distance(a, b, max_distance), expected);
    }
}
