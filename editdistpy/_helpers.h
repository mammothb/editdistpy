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

#ifndef EDITDISTPY__HELPERS_H_
#define EDITDISTPY__HELPERS_H_

#include <cstring>

#include "_def.h"

#ifdef __cplusplus
extern "C" {
#endif

inline int NullDistanceResults(const int *pString1, const int *pString2,
                               int stringLen1, int stringLen2,
                               const int64_t maxDistance) {
  if (pString1 == nullptr) {
    if (pString2 == nullptr) {
      return 0;
    }
    return stringLen2 <= maxDistance ? stringLen2 : -1;
  }
  return stringLen1 <= maxDistance ? stringLen1 : -1;
}

inline int ZeroDistanceResults(const int *pString1, const int *pString2,
                               int stringLen1, int stringLen2) {
  if (stringLen1 != stringLen2) {
    return -1;
  }
  return memcmp(pString1, pString2, stringLen1 * sizeof(int)) == 0 ? 0 : -1;
}

#ifdef __cplusplus
}
#endif

#endif // EDITDISTPY__HELPERS_H_
