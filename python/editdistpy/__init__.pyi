class _levenshtein:
    def distance(
        self, string_1: str | None, string_2: str | None, max_distance: int
    ) -> int: ...

class _damerau_osa:
    def distance(
        self, string_1: str | None, string_2: str | None, max_distance: int
    ) -> int: ...

levenshtein: _levenshtein
damerau_osa: _damerau_osa
