# distutils: language = c++
# distutils: sources = editdistpy/_damerau_osa.cpp
# cython: language_level=3

from libc.stdlib cimport malloc, free

cdef extern from "_damerau_osa.h":
    ctypedef int int64_t
    int Distance(
        const int* pString1,
        const int* pString2,
        int stringLen1,
        int stringLen2,
        const int64_t maxDistance,
    )

cpdef int distance(
    object string_1,
    object string_2,
    object max_distance,
) except +:
    cdef int len_1 = 0
    cdef int* c_string_1 = NULL
    cdef int len_2 = 0
    cdef int* c_string_2 = NULL

    if string_1 is not None:
        len_1 = len(string_1)
        c_string_1 = <int*>malloc(len_1 * sizeof(int))
        for i in range(len_1):
            c_string_1[i] = ord(string_1[i])
    if string_2 is not None:
        len_2 = len(string_2)
        c_string_2 = <int*>malloc(len_2 * sizeof(int))
        for i in range(len_2):
            c_string_2[i] = ord(string_2[i])

    dist = Distance(c_string_1, c_string_2, len_1, len_2, max_distance)
    free(c_string_1)
    free(c_string_2)
    return dist
