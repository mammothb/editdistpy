#ifndef EDITDISTPY__DAMERAU_OSA_HPP_
#define EDITDISTPY__DAMERAU_OSA_HPP_

#include "def.hpp"

#ifdef __cplusplus
extern "C" {
#endif

int Distance(
    const int* string_1
  , const int* string_2
  , const int string_len_1
  , const int string_len_2
  , const int64_t max_distance);

int InternalDistance(
    const int* string_1
  , const int* string_2
  , const int len_1
  , const int len_2
  , const int start);

int InternalDistanceMax(
    const int* string_1
  , const int* string_2
  , const int len_1
  , const int len_2
  , const int start
  , const int64_t max_distance);

#ifdef __cplusplus
}
#endif

#endif  // EDITDISTPY__DAMERAU_OSA_HPP_
