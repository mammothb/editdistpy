# distutils: language = c++
# distutils: sources = editdistpy/_damerau_osa.cpp

cdef extern from "Python.h":
    const char* PyUnicode_AsUTF8(object unicode)

cdef extern from "_damerau_osa.hpp":
    ctypedef int int64_t
    int Distance(
        const char* string_1,
        const char* string_2,
        const int64_t max_distance,
    )

cpdef int distance(
    object string_1,
    object string_2,
    object max_distance
) except +:
    cdef const char* c_string_1 = PyUnicode_AsUTF8(string_1)
    cdef const char* c_string_2 = PyUnicode_AsUTF8(string_2)
    dist = Distance(c_string_1, c_string_2, max_distance)
    return dist
