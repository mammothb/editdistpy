#ifndef EDITDISTPY__HELPERS_HPP_
#define EDITDISTPY__HELPERS_HPP_

#include "def.hpp"

#ifdef __cplusplus
extern "C" {
#endif

void PrefixSuffixPrep(
    const int* string_1
  , const int* string_2
  , int& len_1
  , int& len_2
  , int& start);

#ifdef __cplusplus
}
#endif

#endif  // EDITDISTPY__HELPERS_HPP_
