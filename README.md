editdistpy <br>
[![PyPI version](https://badge.fury.io/py/editdistpy.svg)](https://badge.fury.io/py/editdistpy)
[![Tests](https://github.com/mammothb/editdistpy/actions/workflows/tests.yml/badge.svg)](https://github.com/mammothb/editdistpy/actions/workflows/tests.yml)
========

editdistpy is a fast implementation of the Levenshtein edit distance and
the Damerau-Levenshtein optimal string alignment (OSA) edit distance
algorithms. The original C# project can be found at [SoftWx.Match](https://github.com/softwx/SoftWx.Match).

## Installation

The easiest way to install editdistpy is using `pip`:
```
pip install -U editdistpy
```

## Usage

You can specify the `max_distance` you care about, if the edit distance exceeds
this `max_distance`, `-1` will be returned. Specifying a sensible max distance
can result in significant speed improvement.

You can also specify `max_distance=sys.maxsize` if you wish for the actual edit
distance to always be computed.

### Levenshtein

```python
import sys

from editdistpy import levenshtein

string_1 = "flintstone"
string_2 = "hanson"

max_distance = 2
print(levenshtein.distance(string_1, string_2, max_distance))
# expected output: -1

max_distance = sys.maxsize
print(levenshtein.distance(string_1, string_2, max_distance))
# expected output: 6
```

### Damerau-Levenshtein OSA

```python
import sys

from editdistpy import damerau_osa

string_1 = "flintstone"
string_2 = "hanson"

max_distance = 2
print(damerau_osa.distance(string_1, string_2, max_distance))
# expected output: -1

max_distance = sys.maxsize
print(damerau_osa.distance(string_1, string_2, max_distance))
# expected output: 6
```

## Benchmark

A simple benchmark was done on Python 3.8.12 against [editdistance](https://github.com/roy-ht/editdistance) which implements the Levenshtein edit distance
algorithm.

The script used by the benchmark can be found [here](https://github.com/mammothb/editdistpy/blob/master/tests/benchmarks.py).

For clarity, the following string pairs were used.

### Single word (completely different)
"xabxcdxxefxgx"<br>
"1ab2cd34ef5g6"

### Single word (similar)
"example" <br>
"samples"

### Single word (identical ending)
"kdeisfnexabxcdxlskdixefxgx"<br>
"xabxcdxlskdixefxgx"

### Short string
"short sentence with words"<br>
"shrtsen tence wit mispeledwords"

### Long string
"Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod rem"<br>
"Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium"

```
single_dif string
        test_damerau_osa               0.3055 usec/pass 611.02 msec total 2000000 iterations
        test_levenshtein               0.3030 usec/pass 606.05 msec total 2000000 iterations
        test_editdistance              0.7036 usec/pass 1407.27 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.3042 usec/pass 608.37 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.2997 usec/pass 599.44 msec total 2000000 iterations
single_sim string
        test_damerau_osa               0.2379 usec/pass 475.80 msec total 2000000 iterations
        test_levenshtein               0.2434 usec/pass 486.90 msec total 2000000 iterations
        test_editdistance              0.4126 usec/pass 825.17 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.2234 usec/pass 446.78 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.2372 usec/pass 474.36 msec total 2000000 iterations
single_end string
        test_damerau_osa               0.2933 usec/pass 586.63 msec total 2000000 iterations
        test_levenshtein               0.2538 usec/pass 507.57 msec total 2000000 iterations
        test_editdistance              0.9376 usec/pass 1875.16 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.2774 usec/pass 554.73 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.2497 usec/pass 499.48 msec total 2000000 iterations
short string
        test_damerau_osa               0.3533 usec/pass 706.55 msec total 2000000 iterations
        test_levenshtein               0.3772 usec/pass 754.42 msec total 2000000 iterations
        test_editdistance              1.1646 usec/pass 2329.30 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.3515 usec/pass 702.97 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.3798 usec/pass 759.67 msec total 2000000 iterations
long string
        test_damerau_osa               1.3348 usec/pass 2669.56 msec total 2000000 iterations
        test_levenshtein               1.1531 usec/pass 2306.22 msec total 2000000 iterations
        test_editdistance              2.8700 usec/pass 5740.06 msec total 2000000 iterations
        test_damerau_osa early_cutoff  1.3383 usec/pass 2676.50 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.8080 usec/pass 1616.03 msec total 2000000 iterations
```

While `max_distance=10` significantly improves the computation time, it may not
be a sensible value in some cases.

editdistpy is also seen to perform better with shorter length strings and can
be the more suitable library if your use case mainly deals with comparing short
strings.

## Changelog

See the [changelog](https://github.com/mammothb/editdistpy/blob/master/CHANGELOG.md) for a history of notable changes to edistdistpy.
