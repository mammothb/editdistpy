#ifndef EDITDISTPY_DEF_H_
#define EDITDISTPY_DEF_H_

#if defined(_WIN32) && _MSC_VER <= 1600
typedef signed __int32 int32_t;
typedef unsigned __int32 uint32_t;
typedef signed __int64 int64_t;
typedef unsigned __int64 uint64_t;
#else
#include <cstdint>
#endif

#endif  // EDITDISTPY_DEF_H_