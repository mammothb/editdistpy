editdistpy <br>
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

### Short string

"short sentence with words"

"shrtsen tence wit mispeledwords"

### Long string

"Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod rem"

"Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium"

```
short string
        test_damerau_osa               0.925678600000083
        test_levenshtein               0.6640075999998771
        test_editdistance              0.9197039000000586
        test_damerau_osa_early_cutoff  0.7028707999998005
        test_levenshtein_early_cutoff  0.5697816000001694
long string
        test_damerau_osa               7.7526998000003005
        test_levenshtein               4.262871200000063
        test_editdistance              1.9676684999999452
        test_damerau_osa_early_cutoff  0.9891195999998672
        test_levenshtein_early_cutoff  0.9085431999997127
```

While `max_distance=10` significantly improves the computation time, it may not
be a sensible value in some cases.

editdistpy is also seen to perform better with shorter length strings and can
be the more suitable library if your use case mainly deals with comparing short
strings.

## Changelog
------------

See the [changelog](https://github.com/mammothb/editdistpy/blob/master/CHANGELOG.md) for a history of notable changes to edistdistpy.
