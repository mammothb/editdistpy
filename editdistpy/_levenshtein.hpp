#ifndef EDITDISTPY__LEVENSHTEIN_HPP_
#define EDITDISTPY__LEVENSHTEIN_HPP_

#include "def.hpp"

#include <string>

#ifdef __cplusplus
extern "C" {
#endif

int _distance(
    const std::string string_1
  , const std::string string_2
  , const int len_1
  , const int len_2
  , const int start);

int _distance_max(
    const std::string string_1
  , const std::string string_2
  , const int len_1
  , const int len_2
  , const int start
  , const int max_distance);

#ifdef __cplusplus
}
#endif

#endif  // EDITDISTPY__LEVENSHTEIN_HPP_
