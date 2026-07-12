pub fn resolve_inputs<'a>(
    s1: Option<&'a str>,
    s2: Option<&'a str>,
    max_distance: i64,
) -> Result<(&'a str, &'a str), i32> {
    match (s1, s2) {
        (None, None) => Err(0),
        (None, Some(s)) | (Some(s), None) => {
            let len = s.chars().count() as i64;
            Err(if len <= max_distance { len as i32 } else { -1 })
        }
        (Some(a), Some(b)) => {
            if max_distance <= 0 {
                return Err(if a == b { 0 } else { -1 });
            }
            Ok((a, b))
        }
    }
}

/// Trim common prefix and suffix char-by-char.
///
/// `v1` must be the **shorter** of the two slices (caller swaps before calling).
/// After trimming, if `v1` is fully consumed, returns the distance (or -1 if
/// `max_distance` exceeded). Otherwise returns the trimmed sub-slices.
pub fn trim_affix<'a>(
    v1: &'a [char],
    v2: &'a [char],
    max_distance: i64,
) -> Result<(&'a [char], &'a [char]), i32> {
    let mut len1 = v1.len();
    let mut len2 = v2.len();
    // suffix
    while len1 > 0 && len2 > 0 && v1[len1 - 1] == v2[len2 - 1] {
        len1 -= 1;
        len2 -= 1;
    }
    if len1 == 0 {
        return Err(if len2 as i64 <= max_distance {
            len2 as i32
        } else {
            -1
        });
    }
    // prefix
    let prefix = v1[..len1]
        .iter()
        .zip(&v2[..len2])
        .take_while(|(a, b)| a == b)
        .count();
    len1 -= prefix;
    len2 -= prefix;
    if len1 == 0 {
        return Err(if len2 as i64 <= max_distance {
            len2 as i32
        } else {
            -1
        });
    }
    Ok((&v1[prefix..prefix + len1], &v2[prefix..prefix + len2]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    // ==============
    // resolve_inputs
    // ==============

    #[rstest]
    #[case(None, None, 10, 0)]
    #[case(None, Some("abc"), 10, 3)]
    #[case(Some("abc"), None, 10, 3)]
    #[case(None, Some("abc"), 2, -1)] // len > max_distance
    #[case(Some("abc"), None, 2, -1)]
    #[case(None, Some("abc"), 3, 3)] // len == max_distance
    #[case(None, Some(""), 0, 0)] // empty string, zero max
    #[case(None, Some(""), 10, 0)] // empty string, roomy max
    #[case(Some("abc"), Some("xyz"), -1, -1)] // negative max, different -> -1
    #[case(Some("abc"), Some("abc"), -1, 0)] // negative max, identical -> 0
    fn test_resolve_inputs_err(
        #[case] s1: Option<&str>,
        #[case] s2: Option<&str>,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        assert_eq!(resolve_inputs(s1, s2, max_distance), Err(expected));
    }

    #[rstest]
    #[case(Some("abc"), Some("xyz"))]
    #[case(Some("abc"), Some("abc"))]
    #[case(Some(""), Some(""))]
    #[case(Some("a"), Some(""))]
    fn test_resolve_inputs_ok(#[case] s1: Option<&str>, #[case] s2: Option<&str>) {
        let result = resolve_inputs(s1, s2, 10).unwrap();
        assert_eq!(result.0, s1.unwrap());
        assert_eq!(result.1, s2.unwrap());
    }

    // ==========
    // trim_affix
    // ==========

    #[rstest]
    #[case("", "", 0, 0)] // both empty -> Err(0)
    #[case("abc", "abc", 0, 0)] // identical -> Err(0)
    #[case("ab", "ab", 0, 0)]
    fn test_trim_affix_identical(
        #[case] a: &str,
        #[case] b: &str,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        let va = chars(a);
        let vb = chars(b);
        assert_eq!(trim_affix(&va, &vb, max_distance), Err(expected));
    }

    #[rstest]
    #[case("abc", "xyz", "abc", "xyz")] // no affix in common
    #[case("abcx", "xyzx", "abc", "xyz")] // suffix only: 'x' trimmed
    #[case("xabc", "xxyz", "abc", "xyz")] // prefix only: 'x' trimmed
    #[case("xabcy", "xxyzy", "abc", "xyz")] // both prefix and suffix
    fn test_trim_affix_shape(
        #[case] a: &str,
        #[case] b: &str,
        #[case] expected_a: &str,
        #[case] expected_b: &str,
    ) {
        let va = chars(a);
        let vb = chars(b);
        let (ta, tb) = trim_affix(&va, &vb, 100).unwrap();
        assert_eq!(ta, chars(expected_a).as_slice());
        assert_eq!(tb, chars(expected_b).as_slice());
    }

    /// v1 consumed entirely during prefix/suffix trim -> Err(distance) early
    /// return. These cases have equal-length strings so v1 is the shorter.
    #[rstest]
    #[case("ab", "abxx",   1, -1)] // v2 remaining len=2 > max=1 -> -1
    #[case("ab", "abxx", 2, 2)] // v2 remaining len=2 == max=2 -> Err(2)
    #[case("abc", "abcxxx", 2, -1)] // v2 remaining len=3 > max=2 -> -1
    #[case("ab", "xxab",   1, -1)] // v2 remaining len=2 > max=1 -> -1
    #[case("ab", "xxab", 2, 2)] // v2 remaining len=2 == max=2 -> Err(2)
    #[case("abc", "xxxabc", 2, -1)] // v2 remaining len=3 > max=2 -> -1
    fn test_trim_affix_consumed(
        #[case] a: &str,
        #[case] b: &str,
        #[case] max_distance: i64,
        #[case] expected: i32,
    ) {
        let va = chars(a);
        let vb = chars(b);
        assert_eq!(trim_affix(&va, &vb, max_distance), Err(expected));
    }

    #[test]
    fn test_trim_affix_unicode() {
        // multi-byte codepoints: trim should work at char level, not byte level
        let va = chars("héllö");
        let vb = chars("héllo");
        let (ta, tb) = trim_affix(&va, &vb, 100).unwrap();
        // common prefix: "héll" (4 chars), remaining differ
        assert_eq!(ta, chars("ö").as_slice());
        assert_eq!(tb, chars("o").as_slice());
    }

    #[test]
    fn test_trim_affix_long_common_prefix() {
        let va = chars("prefixXXXXsuffix");
        let vb = chars("prefixYYYYsuffix");
        let (ta, tb) = trim_affix(&va, &vb, 100).unwrap();
        assert_eq!(ta, chars("XXXX").as_slice());
        assert_eq!(tb, chars("YYYY").as_slice());
    }
}
