import sys
import timeit

import editdistance

from editdistpy import damerau_osa, levenshtein

a = "fsffvfdsbbdfvvdavavavavavava"
b = "fvdaabavvvvvadvdvavavadfsfsdafvvav"


def test_damerau_osa():
    damerau_osa.distance(a, b, sys.maxsize)


def test_damerau_osa_early_cutoff():
    damerau_osa.distance(a, b, 10)


def test_levenshtein():
    levenshtein.distance(a, b, sys.maxsize)


def test_levenshtein_early_cutoff():
    levenshtein.distance(a, b, 10)


def test_editdistance():
    editdistance.eval(a, b)


if __name__ == "__main__":
    for function in (
        "test_damerau_osa",
        "test_levenshtein",
        "test_editdistance",
        "test_damerau_osa_early_cutoff",
        "test_levenshtein_early_cutoff",
    ):
        print(
            function,
            timeit.timeit(f"{function}()", setup=f"from __main__ import {function}"),
        )
