# distutils: language = c++
# distutils: sources = editdistpy/_damerau_osa.cpp

cdef extern from "Python.h":
    const char* PyUnicode_AsUTF8(object unicode)

cdef extern from "_damerau_osa.hpp":
    int _distance(
        const char* string_1,
        const char* string_2,
        const int len_1,
        const int len_2,
        const int start,
    )

    int _distance_max(
        const char* string_1,
        const char* string_2,
        const int len_1,
        const int len_2,
        const int start,
        const int max_distance,
    )

cpdef int distance(
    object string_1,
    object string_2,
    object len_1,
    object len_2,
    object start
) except +:
    cdef const char* c_string_1 = PyUnicode_AsUTF8(string_1)
    cdef const char* c_string_2 = PyUnicode_AsUTF8(string_2)
    dist = _distance(c_string_1, c_string_2, len_1, len_2, start)
    return dist

cpdef int distance_max(
    object string_1,
    object string_2,
    object len_1,
    object len_2,
    object start,
    object max_distance
) except +:
    cdef const char* c_string_1 = PyUnicode_AsUTF8(string_1)
    cdef const char* c_string_2 = PyUnicode_AsUTF8(string_2)
    dist = _distance_max(c_string_1, c_string_2, len_1, len_2, start, max_distance)
    return dist