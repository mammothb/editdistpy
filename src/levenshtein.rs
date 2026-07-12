use crate::common::{resolve_inputs, trim_affix};
use crate::pmv::{BlockPatternMatchVector, PatternMatchVector};

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
    let cutoff = if max_distance < t2.len() as i64 {
        max_distance as usize
    } else {
        usize::MAX
    };
    if t1.len() <= 64 {
        hyrro2001(t1, t2, cutoff)
    } else {
        hyrro2001_block(t1, t2, cutoff)
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

fn hyrro2001(v1: &[char], v2: &[char], score_cutoff: usize) -> i32 {
    let len1 = v1.len();
    let len2 = v2.len();

    let pm = PatternMatchVector::new(v1);
    let last_row_mask = 1u64 << (len1 - 1);

    let mut vp = !0u64; // all vertical deltas = +1 initially
    let mut vn = 0u64;
    let mut dist = len1; // score = m at column 0

    // Budget for remaining misses; decreases monotonically.
    // When depleted the distance cannot recover below score_cutoff.
    let bounded = score_cutoff < usize::MAX;
    let mut max_misses = if bounded {
        score_cutoff + len2 - len1
    } else {
        0 // unused in unbounded mode
    };

    for &c2 in v2 {
        let eq = pm.get(c2);

        // D0: diagonal-zero mask via the carry trick
        let d0 = ((eq & vp).wrapping_add(vp) ^ vp) | eq | vn;

        // Horizontal deltas
        let mut hp = vn | !(d0 | vp);
        let mut hn = d0 & vp;

        // Score update with max_misses budget tracking
        let hp_hit = hp & last_row_mask != 0;
        let hn_hit = hn & last_row_mask != 0;

        if hp_hit {
            if bounded && max_misses < 2 {
                return -1;
            }
            if bounded {
                max_misses -= 2;
            }
            dist += 1;
        } else if hn_hit {
            dist -= 1;
        } else {
            if bounded && max_misses < 1 {
                return -1;
            }
            if bounded {
                max_misses -= 1;
            }
        }

        // Shift down: row i becomes row i+1
        hp = (hp << 1) | 1; // top boundary: 0-delta
        hn <<= 1;

        // Next column's vertical deltals
        vp = hn | !(d0 | hp);
        vn = hp & d0;
    }
    if dist <= score_cutoff {
        dist as i32
    } else {
        -1
    }
}

/// 0-indexed row number of the last row in `block_idx`.
fn row_num(block_idx: usize, block_count: usize, len1: usize) -> usize {
    if block_idx + 1 == block_count {
        len1 - 1
    } else {
        (block_idx + 1) * 64 - 1
    }
}

fn hyrro2001_block(v1: &[char], v2: &[char], score_cutoff: usize) -> i32 {
    let len1 = v1.len();
    let len2 = v2.len();

    let pm = BlockPatternMatchVector::new(v1);
    let block_count = pm.size();
    let last = 1u64 << ((len1 - 1) % 64);

    // Per-block state: vertical deltas and distance at each block boundary
    let mut vp = vec![!0u64; block_count];
    let mut vn = vec![0u64; block_count];
    let mut scores: Vec<_> = (0..block_count)
        .map(|b| {
            if b + 1 == block_count {
                len1
            } else {
                (b + 1) * 64
            }
        })
        .collect();

    // Initial band width (len2 >= len1 after swap, so len2 - len1 is safe)
    let len_diff = len2 - len1;
    let band_half = score_cutoff.saturating_add(len_diff) / 2;
    let band_width = (score_cutoff.min(band_half) + 1) / 64;
    let mut block_start = 0;
    let mut block_end = block_count.min(band_width + 1).saturating_sub(1);

    let score_cutoff = score_cutoff.min(len2);

    for (j, &c2) in v2.iter().enumerate() {
        let mut hp_carry = 1u64; // 0-delta at row 0 (top boundary)
        let mut hn_carry = 0u64;

        // Process active block
        for block_idx in block_start..=block_end {
            let eq = pm.get(block_idx, c2);

            // D0: hn_carry OR'd into x, propagates carry into the bitwise formula
            let x = eq | hn_carry;
            let d0 = ((x & vp[block_idx]).wrapping_add(vp[block_idx]) ^ vp[block_idx])
                | x
                | vn[block_idx];

            let mut hp = vn[block_idx] | !(d0 | vp[block_idx]);
            let mut hn = d0 & vp[block_idx];

            // Update score: last row is last bit (last block) or bit 63 (interior)
            if block_idx + 1 == block_count {
                scores[block_idx] += (hp & last != 0) as usize;
                scores[block_idx] -= (hn & last != 0) as usize;
            } else {
                scores[block_idx] += (hp >> 63) as usize;
                scores[block_idx] -= (hn >> 63) as usize;
            }

            // Carry-out: bit 63 propagates to next block's shift
            let hp_carry_out = hp >> 63;
            let hn_carry_out = hn >> 63;

            hp = (hp << 1) | hp_carry;
            hn = (hn << 1) | hn_carry;

            vp[block_idx] = hn | !(d0 | hp);
            vn[block_idx] = hp & d0;

            hp_carry = hp_carry_out;
            hn_carry = hn_carry_out;
        }

        // Band expansion (right)
        if block_end + 1 < block_count {
            let rn = row_num(block_end, block_count, len1);
            if rn as isize
                <= score_cutoff as isize + 2 * 64_isize + j as isize + len1 as isize
                    - scores[block_end] as isize
                    - 2
                    - len2 as isize
            {
                block_end += 1;
                vp[block_end] = !0u64;
                vn[block_end] = 0u64;

                let chars_in_block = if block_end + 1 == block_count {
                    (len1 - 1) % 64 + 1
                } else {
                    64
                };
                scores[block_end] =
                    scores[block_end - 1] + chars_in_block - hp_carry as usize + hn_carry as usize;

                // Process newly added block for current column
                let eq = pm.get(block_end, c2);
                let x = eq | hn_carry;
                let d0 = ((x & vp[block_end]).wrapping_add(vp[block_end]) ^ vp[block_end])
                    | x
                    | vn[block_end];
                let mut hp = vn[block_end] | !(d0 | vp[block_end]);
                let mut hn = d0 & vp[block_end];

                if block_end + 1 == block_count {
                    scores[block_end] += (hp & last != 0) as usize;
                    scores[block_end] -= (hn & last != 0) as usize;
                } else {
                    scores[block_end] += (hp >> 63) as usize;
                    scores[block_end] -= (hn >> 63) as usize;
                }

                hp = (hp << 1) | hp_carry;
                hn = (hn << 1) | hn_carry;
                vp[block_end] = hn | !(d0 | hp);
                vn[block_end] = hp & d0;
            }
        }

        // Band shrinking
        while block_end >= block_start {
            let in_band_cond1 = scores[block_end] < score_cutoff + 64;
            let in_band_cond2 = row_num(block_end, block_count, len1) as isize
                <= score_cutoff as isize + 2 * 64_isize + j as isize + len1 as isize + 1
                    - scores[block_end] as isize
                    - 2
                    - len2 as isize;
            if in_band_cond1 && in_band_cond2 {
                break;
            }
            block_end = block_end.saturating_sub(1);
        }

        while block_start <= block_end {
            let in_band_cond1 = scores[block_start] < score_cutoff + 64;
            let in_band_cond2 = row_num(block_start, block_count, len1) as isize
                >= scores[block_start] as isize + len1 as isize + j as isize
                    - score_cutoff as isize
                    - len2 as isize;
            if in_band_cond1 && in_band_cond2 {
                break;
            }
            block_start += 1;
        }

        if block_end < block_start {
            return -1; // band empty, no match possible
        }
    }
    let dist = scores[block_count - 1];
    if dist <= score_cutoff {
        dist as i32
    } else {
        -1
    }
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rstest::{fixture, rstest};

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
        if (dist as i64) <= max_distance {
            dist
        } else {
            -1
        }
    }

    #[fixture]
    fn strings() -> Vec<String> {
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

    fn random_string(len: usize, seed: u64) -> String {
        let alphabet = b"abcdefghijklmnopqrstuvwxyz";
        let mut rng = StdRng::seed_from_u64(seed);
        (0..len)
            .map(|_| alphabet[rng.random_range(0..26)] as char)
            .collect()
    }

    #[fixture]
    fn long_string_pairs() -> Vec<(String, String)> {
        (0..30)
            .map(|i| {
                let mid_len = 65 + (i * 13) % 136;
                let mid_a = random_string(mid_len, i as u64 * 2);
                let mid_b = random_string(mid_len, i as u64 * 2 + 1);
                (format!("a{}a", mid_a), format!("b{}b", mid_b))
            })
            .collect()
    }

    #[rstest]
    #[case(0)]
    #[case(1)] // fujimoto2018
    #[case(2)] // fujimoto2018
    #[case(3)] // fujimoto2018
    #[case(4)] // internal_distance
    #[case(i32::MAX as i64)]
    fn test_against_reference(#[case] max_distance: i64, strings: Vec<String>) {
        // let strings = permuted_strings();
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

    #[rstest]
    #[case("abc", "abc", 0, 0)]
    #[case("abc", "abd", 0, -1)]
    #[case("abc", "abd", 1, 1)]
    #[case("abc", "xyz", 2, -1)]
    fn test_max_distance_cutoff(
        #[case] a: &str,
        #[case] b: &str,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        assert_eq!(distance(Some(a), Some(b), max_distance), expected);
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

    // ===================================
    // hyrro2001_block coverage (len > 64)
    // ===================================

    #[rstest]
    #[case(0)]
    #[case(7)]
    #[case(100)]
    #[case(300)]
    #[case(i64::MAX as usize)]
    fn test_hyrro2001_block_against_reference(
        #[case] cutoff: usize,
        long_string_pairs: Vec<(String, String)>,
    ) {
        for (s1, s2) in &long_string_pairs {
            let expected = reference_lev(s1, s2, cutoff as i64);
            let actual = distance(Some(s1), Some(s2), cutoff as i64);
            assert_eq!(
                actual, expected,
                "lev({:?}, {:?}, {}) = {}, expected {}",
                s1, s2, cutoff, actual, expected
            );
        }
    }

    #[rstest]
    fn test_hyrro2001_block_band_exhaustion() {
        // Disjoint strings: band must shrink to empty
        let s1 = "x".repeat(130);
        let s2 = "y".repeat(130);
        assert_eq!(distance(Some(&s1), Some(&s2), 3), -1);

        // Identical long strings: distance zero
        let s = "a".repeat(100);
        assert_eq!(distance(Some(&s), Some(&s), 0), 0);
        assert_eq!(distance(Some(&s), Some(&s), 5), 0);
    }
}
