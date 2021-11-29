import sys
import timeit

import editdistance

from editdistpy import damerau_osa, levenshtein

single_dif = ("xabxcdxxefxgx", "1ab2cd34ef5g6")
single_sim = ("example", "samples")
single_end = ("kdeisfnexabxcdxlskdixefxgx", "xabxcdxlskdixefxgx")
short = ("short sentence with words", "shrtsen tence wit mispeledwords")
long = (
    "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod rem",
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium",
)


def test_damerau_osa(s1, s2, max_distance):
    damerau_osa.distance(s1, s2, max_distance)


def test_levenshtein(s1, s2, max_distance):
    levenshtein.distance(s1, s2, max_distance)


def test_editdistance(s1, s2, _):
    editdistance.eval(s1, s2)


def show_results(name, result, count):
    print(name, end=" ")

    per_pass = 1000000 * (result / count)
    print(f"{per_pass:.4f} usec/pass", end=" ")

    result *= 1000
    print(f"{result:.2f} msec total", end=" ")

    print(f"{count} iterations")


if __name__ == "__main__":
    number = 2000000
    for suffix in ("single_dif", "single_sim", "single_end", "short", "long"):
        print(f"{suffix} string")
        for function in (
            "test_damerau_osa ",
            "test_levenshtein ",
            "test_editdistance ",
            "test_damerau_osa early_cutoff",
            "test_levenshtein early_cutoff",
        ):
            if "early_cutoff" in function:
                max_distance = 10
            else:
                max_distance = sys.maxsize
            callable = function[: function.index(" ")]
            show_results(
                f"\t{function:<30}",
                timeit.timeit(
                    f"{callable}({suffix}[0], {suffix}[1], {max_distance})",
                    setup=f"from __main__ import {callable}, {suffix}",
                    number=number,
                ),
                number,
            )
