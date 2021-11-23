#ifndef EDITDISTPY__HELPERS_HPP_
#define EDITDISTPY__HELPERS_HPP_

#include "def.hpp"

#ifdef __cplusplus
extern "C" {
#endif

void PrefixSuffixPrep(
    const char* string_1
  , const char* string_2
  , int& len_1
  , int& len_2
  , int& start);

#ifdef __cplusplus
}
#endif

#endif  // EDITDISTPY__HELPERS_HPP_
