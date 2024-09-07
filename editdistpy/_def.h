/**
 * MIT License
 *
 * Copyright (c) 2024 mmb L
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

#ifndef EDITDISTPY__DEF_H_
#define EDITDISTPY__DEF_H_

#if defined(_WIN32) && _MSC_VER <= 1600
typedef signed __int32 int32_t;
typedef unsigned __int32 uint32_t;
typedef signed __int64 int64_t;
typedef unsigned __int64 uint64_t;
#else
#include <cstdint>
#endif

#endif // EDITDISTPY__DEF_H_
