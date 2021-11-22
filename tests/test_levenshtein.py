import numpy as np
from symspellpy.helpers import null_distance_results, prefix_suffix_prep

from editdistpy import levenshtein


def expected_levenshtein(string_1, string_2, max_distance):
    max_distance = int(min(2 ** 31 - 1, max_distance))
    len_1 = len(string_1)
    len_2 = len(string_2)
    d = np.zeros((len_1 + 1, len_2 + 1))
    for i in range(len_1 + 1):
        d[i, 0] = i
    for i in range(len_2 + 1):
        d[0, i] = i
    for j in range(1, len_2 + 1):
        for i in range(1, len_1 + 1):
            if string_1[i - 1] == string_2[j - 1]:
                # no operation
                d[i, j] = d[i - 1, j - 1]
            else:
                d[i, j] = min(
                    min(d[i - 1, j] + 1, d[i, j - 1] + 1), d[i - 1, j - 1] + 1
                )
    distance = d[len_1, len_2]
    return distance if distance <= max_distance else -1


def actual_levenshtein(string_1, string_2, max_distance):
    if string_1 is None or string_2 is None:
        return null_distance_results(string_1, string_2, max_distance)
    if max_distance <= 0:
        return 0 if string_1 == string_2 else -1
    max_distance = int(min(2 ** 31 - 1, max_distance))
    # if strings of different lengths, ensure shorter string is in string_1.
    # This can result in a little faster speed by spending more time spinning
    # just the inner loop during the main processing.
    if len(string_1) > len(string_2):
        string_2, string_1 = string_1, string_2
    if len(string_2) - len(string_1) > max_distance:
        return -1
    # identify common suffic and/or prefix that can be ignored
    len_1, len_2, start = prefix_suffix_prep(string_1, string_2)
    if len_1 == 0:
        return len_2 if len_2 <= max_distance else -1

    if max_distance < len_2:
        return levenshtein.distance_max(
            string_1,
            string_2,
            len_1,
            len_2,
            start,
            max_distance,
        )
    return levenshtein.distance(string_1, string_2, len_1, len_2, start)


class TestLevenshtein:
    def test_compare_match_ref(self, get_strings):
        strings, max_distance = get_strings

        for s1 in strings:
            for s2 in strings:
                assert expected_levenshtein(s1, s2, max_distance) == actual_levenshtein(
                    s1, s2, max_distance
                )
