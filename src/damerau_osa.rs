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

    if max_distance < t2.len() as i64 {
        internal_distance_max(t1, t2, max_distance)
    } else {
        internal_distance(t1, t2)
    }
}

fn internal_distance(v1: &[char], v2: &[char]) -> i32 {
    let len2 = v2.len();

    let mut prev_char_1_costs = vec![0; len2];
    let mut char_1_costs: Vec<_> = (0..len2).map(|i| i as i32 + 1).collect();

    let mut char_1 = ' ';
    let mut curr_cost = 0;

    for (i, &c1) in v1.iter().enumerate() {
        let prev_char_1 = char_1;
        char_1 = c1;

        let mut char_2 = ' ';
        let mut above_char_cost = i as i32;
        let mut left_char_cost = i as i32;
        let mut next_trans_cost = 0;

        for j in 0..len2 {
            let this_trans_cost = next_trans_cost;
            next_trans_cost = prev_char_1_costs[j];

            // cost of diagnol (substitution)
            curr_cost = left_char_cost;
            prev_char_1_costs[j] = left_char_cost;

            // left now equals current cost (which will be diagnol at next
            // iteration)
            left_char_cost = char_1_costs[j];
            let prev_char_2 = char_2;
            char_2 = v2[j];
            if char_1 != char_2 {
                if above_char_cost < curr_cost {
                    curr_cost = above_char_cost;
                }
                if left_char_cost < curr_cost {
                    curr_cost = left_char_cost;
                }
                curr_cost += 1;

                if i != 0
                    && j != 0
                    && char_1 == prev_char_2
                    && prev_char_1 == char_2
                    && this_trans_cost + 1 < curr_cost
                {
                    // transposition
                    curr_cost = this_trans_cost + 1;
                }
            }
            above_char_cost = curr_cost;
            char_1_costs[j] = curr_cost;
        }
    }
    curr_cost
}

fn internal_distance_max(v1: &[char], v2: &[char], max_distance: i64) -> i32 {
    let len1 = v1.len();
    let len2 = v2.len();
    let max = max_distance as i32;

    let mut prev_char_1_costs = vec![0; len2];
    let mut char_1_costs: Vec<_> = (0..len2).map(|i| i as i32 + 1).collect();

    let len_diff = len2 - len1;
    let j_start_offset = max - len_diff as i32;
    let mut j_start = 0;
    let mut j_end = max as usize;

    let mut char_1 = ' ';
    let mut curr_cost = 0;

    for i in 0..len1 {
        let prev_char_1 = char_1;
        char_1 = v1[i];

        let mut char_2 = ' ';
        let mut above_char_cost = i as i32;
        let mut left_char_cost = i as i32;
        let mut next_trans_cost = 0;

        // no need to look beyond window of lower right diagnol - max_distance
        // cells (lower right diag is i - len_diff) and upper left diagonal +
        // max_distance cells (upper left is i)
        if i as i32 > j_start_offset {
            j_start += 1;
        }
        if j_end < len2 {
            j_end += 1;
        }

        for j in j_start..j_end {
            let this_trans_cost = next_trans_cost;
            next_trans_cost = prev_char_1_costs[j];

            // cost of diagnol (substitution)
            curr_cost = left_char_cost;
            prev_char_1_costs[j] = left_char_cost;

            // left now equals current cost (which will be diagnol at next
            // iteration)
            left_char_cost = char_1_costs[j];
            let prev_char_2 = char_2;
            char_2 = v2[j];
            if char_1 != char_2 {
                if above_char_cost < curr_cost {
                    curr_cost = above_char_cost;
                }
                if left_char_cost < curr_cost {
                    curr_cost = left_char_cost;
                }
                curr_cost += 1;

                if i != 0
                    && j != 0
                    && char_1 == prev_char_2
                    && prev_char_1 == char_2
                    && this_trans_cost + 1 < curr_cost
                {
                    // transposition
                    curr_cost = this_trans_cost + 1;
                }
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

    /// Reference Damerau-Levenshtein OSA
    fn reference_osa(s1: &str, s2: &str, max_distance: i64) -> i32 {
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
                // transposition
                if i > 1 && j > 1 && v1[i - 1] == v2[j - 2] && v1[i - 2] == v2[j - 1] {
                    d[i][j] = d[i][j].min(d[i - 2][j - 2] + cost);
                }
            }
        }
        let dist = d[len1][len2];
        if dist <= max_distance as i32 {
            dist
        } else {
            -1
        }
    }

    /// All permutations of alphabet "abcd", plus empty string (65 strings).
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
    #[case(1)]
    #[case(3)]
    #[case(i32::MAX as i64)]
    fn test_against_reference(#[case] max_distance: i64) {
        let strings = permuted_strings();
        for s1 in &strings {
            for s2 in &strings {
                let expected = reference_osa(s1, s2, max_distance);
                let actual = distance(Some(s1), Some(s2), max_distance);
                assert_eq!(
                    actual, expected,
                    "osa({:?}, {:?}, {}) = {}, expected {}",
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
    #[case("CA", "ABC", 10, 3)] // classic OSA vs DL difference
    #[case("CA", "AC", 10, 1)] // single transposition
    #[case("abc", "acb", 10, 1)] // adjacent swap
    #[case("abcd", "acbd", 10, 1)] // swap in middle
    #[case("flintstone", "hanson", 10, 6)]
    #[case("flintstone", "hanson", 2, -1)]
    #[case("kitten", "sitting", 10, 3)]
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
        // identical strings but max_distance=0 should return 0
        assert_eq!(distance(Some("abc"), Some("abc"), 0), 0);
        // different strings with max_distance=0 should return -1
        assert_eq!(distance(Some("abc"), Some("abd"), 0), -1);
        // within cutoff
        assert_eq!(distance(Some("abc"), Some("abd"), 1), 1);
        // exactly at cutoff
        assert_eq!(distance(Some("abc"), Some("abd"), 1), 1);
        // one away from cutoff
        assert_eq!(distance(Some("abc"), Some("xyz"), 2), -1);
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
