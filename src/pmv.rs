use ahash::AHashMap;

/// Precomputed equality bitmasks for s1 (<= 64 chars).
///
/// Maps each unique character to a 64-bit mask where bit `i` is 1 iff
/// `s1[i] == c`. Peq table from Myers 1999 Section 3.4
///
/// ASCII chars (< 256) use direct array indexing. Non-ASCII chars fall back to
/// AHashMap.
pub struct PatternMatchVector {
    ascii: [u64; 256],
    map: AHashMap<char, u64>,
}

impl PatternMatchVector {
    pub fn new(s1: &[char]) -> Self {
        debug_assert!(s1.len() <= 64, "PatternMatchVector requires s1 <= 64 chars");
        let mut ascii = [0u64; 256];
        let mut map = AHashMap::new();
        let mut mask: u64 = 1;
        for &c in s1 {
            let u = c as u32;
            if u < 256 {
                ascii[u as usize] |= mask;
            } else {
                *map.entry(c).or_insert(0) |= mask;
            }
            mask <<= 1;
        }
        Self { ascii, map }
    }

    /// Returns the bitmask for character `c`. Returns 0 if `c` never appears
    /// in s1.
    #[inline]
    pub fn get(&self, c: char) -> u64 {
        let u = c as u32;
        if u < 256 {
            self.ascii[u as usize]
        } else {
            self.map.get(&c).copied().unwrap_or(0)
        }
    }
}

/// Precomputed equality bitmasks for s1 (> 64 chars), split into blocks.
///
/// Each 64-character block has its own `[u64; 256]` ASCII array and AHashMap.
/// Block `b` covers positions `b*64 .. min((b+1)*64, s1.len())`.
/// Used by `hyrro2001_block` / `hyrro2003_block` for the Ukkonen band.
pub struct BlockPatternMatchVector {
    ascii: Vec<[u64; 256]>,
    maps: Vec<AHashMap<char, u64>>,
}

impl BlockPatternMatchVector {
    pub fn new(s1: &[char]) -> Self {
        let block_count = s1.len().div_ceil(64);
        let mut ascii = vec![[0u64; 256]; block_count];
        let mut maps = vec![AHashMap::new(); block_count];
        for (i, &c) in s1.iter().enumerate() {
            let block_idx = i / 64;
            let mask = 1u64 << (i % 64);
            let u = c as u32;
            if u < 256 {
                ascii[block_idx][u as usize] |= mask;
            } else {
                *maps[block_idx].entry(c).or_insert(0) |= mask;
            }
        }
        Self { ascii, maps }
    }

    #[inline]
    pub fn get(&self, block_idx: usize, c: char) -> u64 {
        let u = c as u32;
        if u < 256 {
            self.ascii[block_idx][u as usize]
        } else {
            self.maps[block_idx].get(&c).copied().unwrap_or(0)
        }
    }

    pub fn size(&self) -> usize {
        self.ascii.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ==================
    // PatternMatchVector
    // ==================

    #[rstest]
    #[case("", 'a', 0)]
    #[case("a", 'a', 1)]
    #[case("a", 'b', 0)]
    #[case("ab", 'a', 0b01)]
    #[case("ab", 'b', 0b10)]
    #[case("aba", 'a', 0b101)]
    #[case("aba", 'b', 0b010)]
    #[case("aba", 'c', 0)]
    fn pmv_get(#[case] s: &str, #[case] query: char, #[case] expected: u64) {
        let chars: Vec<char> = s.chars().collect();
        let pm = PatternMatchVector::new(&chars);
        assert_eq!(pm.get(query), expected);
    }

    #[rstest]
    #[case('中', '中', 1)]
    #[case('文', '中', 0)]
    fn pmv_non_ascii_get(#[case] c: char, #[case] query: char, #[case] expected: u64) {
        let s = vec![c];
        let pm = PatternMatchVector::new(&s);
        assert_eq!(pm.get(query), expected);
    }

    #[rstest]
    #[case(255u32)] // ÿ, ASCII path (< 256)
    #[case(256u32)] // Ā, non-ASCII path (>= 256)
    fn pmv_ascii_boundary_path(#[case] code: u32) {
        let c = char::from_u32(code).unwrap();
        let s = vec![c];
        let pm = PatternMatchVector::new(&s);
        assert_eq!(pm.get(c), 1);
        assert_eq!(pm.get('a'), 0);
    }

    #[rstest]
    fn pmv_exact_64_chars() {
        let s: Vec<char> = (0..64).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let pm = PatternMatchVector::new(&s);
        assert_eq!(pm.get('a'), (1u64 << 0) | (1u64 << 26) | (1u64 << 52));
        assert_eq!(pm.get('b'), (1u64 << 1) | (1u64 << 27) | (1u64 << 53));
        assert_eq!(pm.get('z'), (1u64 << 25) | (1u64 << 51));
    }

    #[rstest]
    fn pmv_mixed_ascii_unicode() {
        let s: Vec<char> = vec!['a', 'é', '中'];
        let pm = PatternMatchVector::new(&s);
        assert_eq!(pm.get('a'), 0b001);
        assert_eq!(pm.get('é'), 0b010);
        assert_eq!(pm.get('中'), 0b100);
    }

    // =======================
    // BlockPatternMatchVector
    // =======================

    #[rstest]
    #[case(0, 0)]
    #[case(63, 1)]
    #[case(64, 1)]
    #[case(65, 2)]
    #[case(128, 2)]
    fn bpmv_size(#[case] len: usize, #[case] expected: usize) {
        let s: Vec<char> = (0..len).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let pm = BlockPatternMatchVector::new(&s);
        assert_eq!(pm.size(), expected);
    }

    #[rstest]
    fn bpmv_single_block() {
        let s: Vec<char> = (0..64).map(|i| (b'a' + i as u8) as char).collect();
        let pm = BlockPatternMatchVector::new(&s);
        assert_eq!(pm.size(), 1);
        assert_eq!(pm.get(0, 'a'), 1u64 << 0);
        assert_eq!(pm.get(0, (b'a' + 63) as char), 1u64 << 63);
    }

    #[rstest]
    fn bpmv_two_blocks() {
        let s: Vec<char> = (0..65).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let pm = BlockPatternMatchVector::new(&s);
        assert_eq!(pm.size(), 2);
        assert_eq!(pm.get(0, 'a'), (1u64 << 0) | (1u64 << 26) | (1u64 << 52));
        assert_eq!(pm.get(1, 'a'), 0);
        assert_eq!(pm.get(1, 'm'), 1u64 << 0); // position 64, 64 % 26 = 12 = 'm'
    }

    #[rstest]
    fn bpmv_block_boundary_exact() {
        let s: Vec<char> = (0..128).map(|_| 'x').collect();
        let pm = BlockPatternMatchVector::new(&s);
        assert_eq!(pm.size(), 2);
        assert_eq!(pm.get(0, 'x'), u64::MAX);
        assert_eq!(pm.get(1, 'x'), u64::MAX);
    }

    #[rstest]
    fn bpmv_multi_block() {
        let s: Vec<char> = (0..200).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let pm = BlockPatternMatchVector::new(&s);
        assert_eq!(pm.size(), 4);
        assert_eq!(pm.get(0, 'a'), (1u64 << 0) | (1u64 << 26) | (1u64 << 52));
        assert_eq!(pm.get(1, 'a'), (1u64 << 14) | (1u64 << 40));
        assert_eq!(pm.get(2, 'a'), (1u64 << 2) | (1u64 << 28) | (1u64 << 54));
        assert_eq!(pm.get(3, 'a'), 0);
    }

    #[rstest]
    fn bpmv_non_ascii() {
        let s: Vec<char> = (0..100)
            .map(|i| if i % 2 == 0 { '中' } else { '文' })
            .collect();
        let pm = BlockPatternMatchVector::new(&s);
        assert_eq!(pm.size(), 2);
        assert_eq!(pm.get(0, '中'), 0x5555555555555555);
        let expected_block1: u64 = (0..36).step_by(2).fold(0, |acc, i| acc | (1u64 << i));
        assert_eq!(pm.get(1, '中'), expected_block1);
        assert_eq!(pm.get(0, '文'), 0xAAAAAAAAAAAAAAAA);
        let expected_block1_wen: u64 = (1..36).step_by(2).fold(0, |acc, i| acc | (1u64 << i));
        assert_eq!(pm.get(1, '文'), expected_block1_wen);
    }

    #[rstest]
    fn bpmv_mixed_ascii_unicode() {
        let s: Vec<char> = (0..150)
            .map(|i| match i % 3 {
                0 => 'a',
                1 => 'é',
                _ => '中',
            })
            .collect();
        let pm = BlockPatternMatchVector::new(&s);
        assert_eq!(pm.size(), 3);
        let mask_a_block0: u64 = (0..64)
            .filter(|i| i % 3 == 0)
            .fold(0, |acc, i| acc | (1 << i));
        assert_eq!(pm.get(0, 'a'), mask_a_block0);
        let mask_a_block1: u64 = (64..128)
            .filter(|i| i % 3 == 0)
            .fold(0, |acc, i| acc | (1 << (i - 64)));
        assert_eq!(pm.get(1, 'a'), mask_a_block1);
        let mask_cjk_block0: u64 = (0..64)
            .filter(|i| i % 3 == 2)
            .fold(0, |acc, i| acc | (1 << i));
        assert_eq!(pm.get(0, '中'), mask_cjk_block0);
    }
}
