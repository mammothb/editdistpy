# distutils: language = c++
# distutils: sources = editdistpy/_levenshtein.cpp

from libcpp.string cimport string

cdef extern from "_levenshtein.hpp":
    int _distance(
        const string string_1,
        const string string_2,
        const int len_1,
        const int len_2,
        const int start,
    )

    int _distance_max(
        const string string_1,
        const string string_2,
        const int len_1,
        const int len_2,
        const int start,
        const int max_distance,
    )

cpdef int distance(
    str string_1,
    str string_2,
    int len_1,
    int len_2,
    int start
) except +:
    cdef string cpp_string_1 = string_1.encode("UTF-8")
    cdef string cpp_string_2 = string_2.encode("UTF-8")
    dist = _distance(cpp_string_1, cpp_string_2, len_1, len_2, start)
    return dist

cpdef int distance_max(
    str string_1,
    str string_2,
    int len_1,
    int len_2,
    int start,
    int max_distance
) except +:
    cdef string cpp_string_1 = string_1.encode("UTF-8")
    cdef string cpp_string_2 = string_2.encode("UTF-8")
    dist = _distance_max(cpp_string_1, cpp_string_2, len_1, len_2, start, max_distance)
    return dist