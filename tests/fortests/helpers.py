def null_distance_results(string1: str, string2: str, max_distance: int) -> int:
    """Determines the proper return value of an edit distance function when one
    or both strings are null.

    Args:
        string_1: Base string.
        string_2: The string to compare.
        max_distance: The maximum distance allowed.

    Returns:
        -1 if the distance is greater than the max_distance, 0 if the strings are
            equivalent (both are None), otherwise a positive number whose
            magnitude is the length of the string which is not None.
    """
    if string1 is None:
        if string2 is None:
            return 0
        return len(string2) if len(string2) <= max_distance else -1
    return len(string1) if len(string1) <= max_distance else -1
