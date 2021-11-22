import sys
from itertools import combinations, permutations

import pytest


@pytest.fixture(params=[0, 1, 3, sys.maxsize])
def get_strings(request):
    alphabet = "abcd"
    strings = [""]
    for i in range(1, len(alphabet) + 1):
        for combi in combinations(alphabet, i):
            strings += ["".join(p) for p in permutations(combi)]
    yield strings, request.param
