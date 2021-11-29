#ifndef EDITDISTPY__HELPERS_HPP_
#define EDITDISTPY__HELPERS_HPP_

#ifdef __cplusplus
extern "C" {
#endif

#include <cstring>

inline int NullDistanceResults(
    const int* pString1
  , const int* pString2
  , int stringLen1
  , int stringLen2
  , const int64_t maxDistance)
{
  if (pString1 == NULL) {
    if (pString2 == NULL) {
      return 0;
    }
    return stringLen2 <= maxDistance ? stringLen2 : -1;
  }
  return stringLen1 <= maxDistance ? stringLen1 : -1;
}

inline int ZeroDistanceResults(
    const int* pString1
  , const int* pString2
  , int stringLen1
  , int stringLen2)
{
  if (stringLen1 != stringLen2) {
    return -1;
  }
  return memcmp(pString1, pString2, stringLen1 * sizeof(int)) == 0 ? 0 : -1;
}

#ifdef __cplusplus
}
#endif

#endif  // EDITDISTPY__HELPERS_HPP_
