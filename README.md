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

The script used by the benchmark can be found [here](/tests/benchmarks.py).

For clarity, the following string pairs were used.

### Short string

"short sentence with several words"

"shrtsen tence wit some mispeledwords"

### Long string

"Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod rem"

"Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium"

```
short string
        test_damerau_osa               1.1432659000001877
        test_levenshtein               0.5929201000003559
        test_editdistance              1.0418791000001875
        test_damerau_osa_early_cutoff  0.5969067999999425
        test_levenshtein_early_cutoff  0.39980750000040643
long string
        test_damerau_osa               7.43553189999966
        test_levenshtein               3.5767219999997906
        test_editdistance              2.019506300000103
        test_damerau_osa_early_cutoff  0.3622571999999309
        test_levenshtein_early_cutoff  0.27194450000024517
```

While `max_distance=10` significantly improves the computation time, it may not
be a sensible value in some cases.

editdistpy is also seen to perform better with shorter length strings and can
be the more suitable library if your use case mainly deals with comparing short
strings.

## Changelog
------------

See the [changelog] for a history of notable changes to edistdistpy.
