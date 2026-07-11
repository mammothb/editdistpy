"""Fast Levenshtein and Damerau optimal string alignment algorithms."""

from ._editdistpy import damerau_osa_distance, levenshtein_distance


class _DistanceWrapper:
    """Wrapper to expose distance() callable on submodule-like attributes."""

    def __init__(self, func):
        self.distance = func


levenshtein = _DistanceWrapper(levenshtein_distance)
damerau_osa = _DistanceWrapper(damerau_osa_distance)
