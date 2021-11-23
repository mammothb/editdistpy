#ifndef EDITDISTPY__LEVENSHTEIN_HPP_
#define EDITDISTPY__LEVENSHTEIN_HPP_

#include "def.hpp"

#ifdef __cplusplus
extern "C" {
#endif

int Distance(
    const char* string_1
  , const char* string_2
  , const int64_t max_distance);

int InternalDistance(
    const char* string_1
  , const char* string_2
  , const int len_1
  , const int len_2
  , const int start);

int InternalDistanceMax(
    const char* string_1
  , const char* string_2
  , const int len_1
  , const int len_2
  , const int start
  , const int64_t max_distance);

#ifdef __cplusplus
}
#endif

#endif  // EDITDISTPY__LEVENSHTEIN_HPP_
