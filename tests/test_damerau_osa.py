from fortests.helpers import null_distance_results

from editdistpy import damerau_osa


def expected_damerau_osa(string_1, string_2, max_distance):
    max_distance = int(min(2**31 - 1, max_distance))
    len_1 = len(string_1)
    len_2 = len(string_2)
    #  d = np.zeros((len_1 + 1, len_2 + 1))
    d = [[0] * (len_2 + 1) for _ in range(len_1 + 1)]
    for i in range(len_1 + 1):
        d[i][0] = i
    for i in range(len_2 + 1):
        d[0][i] = i
    for i in range(1, len_1 + 1):
        for j in range(1, len_2 + 1):
            cost = 0 if string_1[i - 1] == string_2[j - 1] else 1
            d[i][j] = min(d[i - 1][j] + 1, d[i][j - 1] + 1, d[i - 1][j - 1] + cost)
            if (
                i > 1
                and j > 1
                and string_1[i - 1] == string_2[j - 2]
                and string_1[i - 2] == string_2[j - 1]
            ):
                d[i][j] = min(d[i][j], d[i - 2][j - 2] + cost)
    distance = d[len_1][len_2]
    return distance if distance <= max_distance else -1


def actual_damerau_osa(string_1, string_2, max_distance):
    if string_1 is None or string_2 is None:
        return null_distance_results(string_1, string_2, max_distance)
    if max_distance <= 0:
        return 0 if string_1 == string_2 else -1
    max_distance = int(min(2**31 - 1, max_distance))
    # if strings of different lengths, ensure shorter string is in string_1.
    # This can result in a little faster speed by spending more time spinning
    # just the inner loop during the main processing.
    len_1 = len(string_1)
    len_2 = len(string_2)
    if len_1 > len_2:
        string_2, string_1 = string_1, string_2
        len_2, len_1 = len_1, len_2
    if len_2 - len_1 > max_distance:
        return -1
    return damerau_osa.distance(string_1, string_2, max_distance)


class TestDamerauOsa:
    def test_compare_match_ref(self, get_strings):
        strings, max_distance = get_strings

        for s1 in strings:
            for s2 in strings:
                assert expected_damerau_osa(s1, s2, max_distance) == actual_damerau_osa(
                    s1, s2, max_distance
                )

    def test_compare_match_ref_cython(self, get_strings):
        strings, max_distance = get_strings

        for s1 in strings:
            for s2 in strings:
                assert expected_damerau_osa(
                    s1, s2, max_distance
                ) == damerau_osa.distance(s1, s2, max_distance)

    def test_comparer_null_distance(self, get_short_and_long_strings):
        for s1, s2, expected in get_short_and_long_strings:
            distance = damerau_osa.distance(s1, s2, 10)
            assert expected["null"] == distance
