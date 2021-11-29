import sys
from itertools import combinations, permutations

import pytest

SHORT_STRING = "string"
LONG_STRING = "long_string"
VERY_LONG_STRING = "very_long_string"


@pytest.fixture(params=[0, 1, 3, sys.maxsize])
def get_strings(request):
    alphabet = "abcd"
    strings = [""]
    for i in range(1, len(alphabet) + 1):
        for combi in combinations(alphabet, i):
            strings += ["".join(p) for p in permutations(combi)]
    yield strings, request.param


@pytest.fixture
def get_short_and_long_strings():
    return [
        (SHORT_STRING, None, {"null": len(SHORT_STRING), "zero": -1, "neg": -1}),
        (LONG_STRING, None, {"null": -1, "zero": -1, "neg": -1}),
        (None, SHORT_STRING, {"null": len(SHORT_STRING), "zero": -1, "neg": -1}),
        (None, LONG_STRING, {"null": -1, "zero": -1, "neg": -1}),
        (SHORT_STRING, SHORT_STRING, {"null": 0, "zero": 0, "neg": 0}),
        (None, None, {"null": 0, "zero": 0, "neg": 0}),
    ]
