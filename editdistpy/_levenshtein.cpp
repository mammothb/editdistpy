#include "_levenshtein.hpp"

#include "_helpers.hpp"

int Distance(
    const char* string_1
  , const char* string_2
  , const int64_t max_distance)
{
  int len_1;
  int len_2;
  int start;
  PrefixSuffixPrep(string_1, string_2, len_1, len_2, start);
  if (len_1 == 0) {
    return len_2 <= max_distance ? len_2 : -1;
  }
  if (max_distance < len_2) {
    return InternalDistanceMax(
        string_1,
        string_2,
        len_1,
        len_2,
        start,
        max_distance);
  }
  return InternalDistance(string_1, string_2, len_1, len_2, start);
}

int InternalDistance(
    const char* string_1
  , const char* string_2
  , const int len_1
  , const int len_2
  , const int start)
{
  int char_1_costs[len_2];
  for (int i = 0; i < len_2; ++i) {
    char_1_costs[i] = i + 1;
  }
  int current_cost = 0;
  for (int i = 0; i < len_1; ++i) {
    int above_char_cost = i;
    int left_char_cost = i;
    const char char_1 = string_1[start + i];
    for (int j = 0; j < len_2; ++j) {
      current_cost = left_char_cost;
      left_char_cost = char_1_costs[j];
      if (char_1 != string_2[start + j]) {
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (left_char_cost < current_cost) {
          current_cost = left_char_cost;
        }
        ++current_cost;
      }
      above_char_cost = current_cost;
      char_1_costs[j] = current_cost;
    }
  }
  return current_cost;
}

int InternalDistanceMax(
    const char* string_1
  , const char* string_2
  , const int len_1
  , const int len_2
  , const int start
  , const int64_t max_distance)
{
  int char_1_costs[len_2];
  for (int i = 0; i < len_2; ++i) {
    char_1_costs[i] = i < max_distance ? i + 1 : max_distance + 1;
  }
  const int len_diff = len_2 - len_1;
  const int64_t j_start_offset = max_distance - len_diff;
  int j_start = 0;
  int64_t j_end = max_distance;
  int current_cost = 0;
  for (int i = 0; i < len_1; ++i) {
    char char_1 = string_1[start + i];
    int above_char_cost = i;
    int prev_char_1_cost = i;
    if (i > j_start_offset) {
      ++j_start;
    }
    if (j_end < len_2) {
      ++j_end;
    }
    for (int j = j_start; j < j_end; ++j) {
      current_cost = prev_char_1_cost;
      prev_char_1_cost = char_1_costs[j];
      if (char_1 != string_2[start + j]) {
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (prev_char_1_cost < current_cost) {
          current_cost = prev_char_1_cost;
        }
        ++current_cost;
      }
      above_char_cost = current_cost;
      char_1_costs[j] = current_cost;
    }
    if (char_1_costs[i + len_diff] > max_distance) {
      return -1;
    }
  }
  return current_cost <= max_distance ? current_cost : -1;
}
