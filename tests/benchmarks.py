import sys
import timeit

import editdistance

from editdistpy import damerau_osa, levenshtein

a = "short sentence with words"
b = "shrtsen tence wit mispeledwords"
c = "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod rem"
d = "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium"


def test_damerau_osa_short():
    damerau_osa.distance(a, b, sys.maxsize)


def test_damerau_osa_early_cutoff_short():
    damerau_osa.distance(a, b, 10)


def test_levenshtein_short():
    levenshtein.distance(a, b, sys.maxsize)


def test_levenshtein_early_cutoff_short():
    levenshtein.distance(a, b, 10)


def test_editdistance_short():
    editdistance.eval(a, b)


def test_damerau_osa_long():
    damerau_osa.distance(c, d, sys.maxsize)


def test_damerau_osa_early_cutoff_long():
    damerau_osa.distance(c, d, 10)


def test_levenshtein_long():
    levenshtein.distance(c, d, sys.maxsize)


def test_levenshtein_early_cutoff_long():
    levenshtein.distance(c, d, 10)


def test_editdistance_long():
    editdistance.eval(c, d)


def show_results(name, result, count):
    print(name, end=" ")

    per_pass = 1000000 * (result / count)
    print(f"{per_pass:.4f} usec/pass", end=" ")

    result *= 1000
    print(f"{result:.2f} msec total", end=" ")

    print(f"{count} iterations")


if __name__ == "__main__":
    number = 2000000
    for suffix in ("_short", "_long"):
        print(f"{suffix[1:]} string")
        for function in (
            "test_damerau_osa",
            "test_levenshtein",
            "test_editdistance",
            "test_damerau_osa_early_cutoff",
            "test_levenshtein_early_cutoff",
        ):
            show_results(
                f"\t{function:<30}",
                timeit.timeit(
                    f"{function}{suffix}()",
                    setup=f"from __main__ import {function}{suffix}",
                    number=number,
                ),
                number,
            )
