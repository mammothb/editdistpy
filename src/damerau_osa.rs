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

    // v1 must be longer, it becomes the PMV source.
    // The transposition formula ((!d0)&Eq_j)<<1 & Eq_{j-1} is directional.
    if v1.len() < v2.len() {
        std::mem::swap(&mut v1, &mut v2);
    }
    if v1.len() as i64 - v2.len() as i64 > max_distance {
        return -1;
    }

    let (t1, t2) = match trim_affix(&v1, &v2, max_distance) {
        Ok(pair) => pair,
        Err(distance) => return distance,
    };

    let cutoff = if max_distance < t1.len() as i64 {
        max_distance as usize
    } else {
        usize::MAX
    };
    if t1.len() <= 64 {
        hyrro2003(t1, t2, cutoff)
    } else {
        hyrro2003_block(t1, t2, cutoff)
    }
}

fn hyrro2003(v1: &[char], v2: &[char], score_cutoff: usize) -> i32 {
    let len1 = v1.len();
    let len2 = v2.len();

    let pm = PatternMatchVector::new(v1);
    let last_row_mask = 1u64 << (len1 - 1);

    let mut vp = !0u64;
    let mut vn = 0u64;
    let mut d0 = 0u64; // persists across columns
    let mut pm_j_old = 0u64; // previous column's Eq
    let mut dist = len1;

    let bounded = score_cutoff < usize::MAX;
    let mut max_misses = if bounded {
        score_cutoff + len2 - len1
    } else {
        0
    };

    for &c2 in v2 {
        let pm_j = pm.get(c2);

        // Transposition check: uses d0 from previous column
        let tr = (((!d0) & pm_j) << 1) & pm_j_old;

        // D0: diagonal-zero mask (same carry trick as Levenshtein)
        d0 = ((pm_j & vp).wrapping_add(vp) ^ vp) | pm_j | vn;
        d0 |= tr;

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

        // Shift down
        hp = (hp << 1) | 1;
        hn <<= 1;

        // Next column's vertical deltas
        vp = hn | !(d0 | hp);
        vn = hp & d0;
        pm_j_old = pm_j;
    }

    if dist <= score_cutoff {
        dist as i32
    } else {
        -1
    }
}

/// Per-block state that persists across columns (for transposition detection).
#[derive(Clone, Copy)]
struct Row {
    vp: u64,
    vn: u64,
    d0: u64,
    pm: u64, // pm_j_old for this block
}

impl Default for Row {
    fn default() -> Self {
        Self {
            vp: !0u64,
            vn: 0,
            d0: 0,
            pm: 0,
        }
    }
}

fn hyrro2003_block(v1: &[char], v2: &[char], score_cutoff: usize) -> i32 {
    let len1 = v1.len();

    let pm = BlockPatternMatchVector::new(v1);
    let block_count = pm.size();
    let last = 1u64 << ((len1 - 1) % 64);

    // old_vecs[0] is a zeroed sentinel
    // old_vecs[1..=block_count] map to blocks 0..block_count-1
    let mut old_vecs = vec![Row::default(); block_count + 1];
    let mut new_vecs = vec![Row::default(); block_count + 1];
    let mut dist = len1;

    for &c2 in v2 {
        let mut hp_carry = 1u64;
        let mut hn_carry = 0u64;

        for block_idx in 0..block_count {
            let vn = old_vecs[block_idx + 1].vn;
            let vp = old_vecs[block_idx + 1].vp;
            let d0_old = old_vecs[block_idx + 1].d0; // previous column's D0
            let d0_last = old_vecs[block_idx].d0; // prev block's D0, prev column
            let pm_j_old = old_vecs[block_idx + 1].pm; // prev column's Eq
            let pm_last = new_vecs[block_idx].pm; // prev block's Eq, current column

            let pm_j = pm.get(block_idx, c2);

            // Cross-block transposition
            let tr = ((((!d0_old) & pm_j) << 1) | (((!d0_last) & pm_last) >> 63)) & pm_j_old;

            // D0: hn_carry included in x
            let x = pm_j | hn_carry;
            let d0 = ((x & vp).wrapping_add(vp) ^ vp) | x | vn | tr;

            let mut hp = vn | !(d0 | vp);
            let mut hn = d0 & vp;

            // Score update: only the last block contributes to the final
            // distance. Interior block boundaries aren't the final answer (cf.
            // hyrro2001_block which tracks per-block scores for band pruning,
            // then reads only the last).
            if block_idx + 1 == block_count {
                dist += (hp & last != 0) as usize;
                dist -= (hn & last != 0) as usize;
            }

            // Carry-out
            let hp_carry_out = hp >> 63;
            let hn_carry_out = hn >> 63;
            hp = (hp << 1) | hp_carry;
            hn = (hn << 1) | hn_carry;

            // Store next-column state
            new_vecs[block_idx + 1].vp = hn | !(d0 | hp);
            new_vecs[block_idx + 1].vn = hp & d0;
            new_vecs[block_idx + 1].d0 = d0;
            new_vecs[block_idx + 1].pm = pm_j;

            hp_carry = hp_carry_out;
            hn_carry = hn_carry_out;
        }

        std::mem::swap(&mut new_vecs, &mut old_vecs);
    }
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
        if (dist as i64) <= max_distance {
            dist
        } else {
            -1
        }
    }

    /// All permutations of alphabet "abcd", plus empty string (65 strings).
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
    #[case(1)]
    #[case(3)]
    #[case(i32::MAX as i64)]
    fn test_against_reference(#[case] max_distance: i64, strings: Vec<String>) {
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
    // hyrro2003_block coverage (len > 64)
    // ===================================

    #[rstest]
    #[case(0)]
    #[case(7)]
    #[case(100)]
    #[case(300)]
    #[case(i64::MAX as usize)]
    fn test_hyrro2003_block_against_reference(
        #[case] cutoff: usize,
        long_string_pairs: Vec<(String, String)>,
    ) {
        for (s1, s2) in &long_string_pairs {
            let expected = reference_osa(s1, s2, cutoff as i64);
            let actual = distance(Some(s1), Some(s2), cutoff as i64);
            assert_eq!(
                actual, expected,
                "osa({:?}, {:?}, {}) = {}, expected {}",
                s1, s2, cutoff, actual, expected
            );
        }
    }

    #[rstest]
    fn test_hyrro2003_block_band_exhaustion() {
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
