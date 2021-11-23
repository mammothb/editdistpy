#include "_helpers.hpp"

#include <cstring>

void PrefixSuffixPrep(
    const char* string_1
  , const char* string_2
  , int& len_1
  , int& len_2
  , int& start)
{
  len_1 = strlen(string_1);
  len_2 = strlen(string_2);
  while (len_1 != 0 && string_1[len_1 - 1] == string_2[len_2 - 1]) {
    --len_1;
    --len_2;
  }
  // prefix common to both strings can be ignored
  start = 0;
  while (start != len_1 && string_1[start] == string_2[start]) {
    ++start;
  }
  if (start != 0) {
    // length of the part excluding common prefix and suffix
    len_1 -= start;
    len_2 -= start;
  }
}
