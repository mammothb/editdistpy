editdistpy <br>
[![PyPI version](https://badge.fury.io/py/editdistpy.svg)](https://badge.fury.io/py/editdistpy)
[![Tests](https://github.com/mammothb/editdistpy/actions/workflows/tests.yml/badge.svg)](https://github.com/mammothb/editdistpy/actions/workflows/tests.yml)
========

editdistpy is a fast implementation of the Levenshtein edit distance and
the Damerau-Levenshtein optimal string alignment (OSA) edit distance
algorithms. The original C# project can be found at [SoftWx.Match](https://github.com/softwx/SoftWx.Match).

## Installation
---------------

The easiest way to install editdistpy is using `pip`:
```
pip install -U editdistpy
```

## Usage
--------

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
------------

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
        test_damerau_osa               0.5202 usec/pass 1040.36 msec total 2000000 iterations
        test_levenshtein               0.3547 usec/pass 709.40 msec total 2000000 iterations
        test_editdistance              0.6399 usec/pass 1279.81 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.5134 usec/pass 1026.72 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.3862 usec/pass 772.31 msec total 2000000 iterations
single_sim string
        test_damerau_osa               0.2983 usec/pass 596.57 msec total 2000000 iterations
        test_levenshtein               0.2433 usec/pass 486.68 msec total 2000000 iterations
        test_editdistance              0.3942 usec/pass 788.36 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.2865 usec/pass 572.90 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.2363 usec/pass 472.61 msec total 2000000 iterations
single_end string
        test_damerau_osa               0.3332 usec/pass 666.32 msec total 2000000 iterations
        test_levenshtein               0.3300 usec/pass 659.93 msec total 2000000 iterations
        test_editdistance              0.7902 usec/pass 1580.42 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.3199 usec/pass 639.74 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.3205 usec/pass 641.01 msec total 2000000 iterations
short string
        test_damerau_osa               0.9925 usec/pass 1984.97 msec total 2000000 iterations
        test_levenshtein               0.6379 usec/pass 1275.76 msec total 2000000 iterations
        test_editdistance              0.9587 usec/pass 1917.37 msec total 2000000 iterations
        test_damerau_osa early_cutoff  0.7535 usec/pass 1506.91 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.5794 usec/pass 1158.79 msec total 2000000 iterations
long string
        test_damerau_osa               8.6244 usec/pass 17248.73 msec total 2000000 iterations
        test_levenshtein               4.2367 usec/pass 8473.36 msec total 2000000 iterations
        test_editdistance              2.0407 usec/pass 4081.31 msec total 2000000 iterations
        test_damerau_osa early_cutoff  1.0795 usec/pass 2158.99 msec total 2000000 iterations
        test_levenshtein early_cutoff  0.9031 usec/pass 1806.28 msec total 2000000 iterations
```

While `max_distance=10` significantly improves the computation time, it may not
be a sensible value in some cases.

editdistpy is also seen to perform better with shorter length strings and can
be the more suitable library if your use case mainly deals with comparing short
strings.

## Changelog
------------

See the [changelog](https://github.com/mammothb/editdistpy/blob/master/CHANGELOG.md) for a history of notable changes to edistdistpy.
