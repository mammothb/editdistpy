"""Fast Levenshtein and Damerau optimal string alignment algorithms."""

from collections.abc import Callable

from ._editdistpy import damerau_osa_distance, levenshtein_distance


class _DistanceWrapper:
    """Wrapper to expose distance() callable on submodule-like attributes."""

    distance: Callable[[str | None, str | None, int], int]

    def __init__(self, func: Callable[[str | None, str | None, int], int]) -> None:
        self.distance = func


levenshtein = _DistanceWrapper(levenshtein_distance)
damerau_osa = _DistanceWrapper(damerau_osa_distance)
