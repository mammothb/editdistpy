#ifndef EDITDISTPY__LEVENSHTEIN_HPP_
#define EDITDISTPY__LEVENSHTEIN_HPP_

#include "def.hpp"

#ifdef __cplusplus
extern "C" {
#endif

int Distance(
    const int* pString1
  , const int* pString2
  , int stringLen1
  , int stringLen2
  , const int64_t maxDistance);

int InternalDistance(
    const int* pString1
  , const int* pString2
  , const int len1
  , const int len2
  , const int start);

int InternalDistanceMax(
    const int* pString1
  , const int* pString2
  , const int len1
  , const int len2
  , const int start
  , const int64_t maxDistance);

#ifdef __cplusplus
}
#endif

#endif  // EDITDISTPY__LEVENSHTEIN_HPP_
