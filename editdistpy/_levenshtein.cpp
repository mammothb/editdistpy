#include "_levenshtein.hpp"

#include <vector>

int _distance(
    const std::string string_1
  , const std::string string_2
  , const int len_1
  , const int len_2
  , const int start)
{
  std::vector<int> char_1_costs(len_2);
  for (int i = 0; i < len_2; ++i) {
    char_1_costs[i] = i + 1;
  }
  int current_cost = 0;
  for (int i = 0; i < len_1; ++i) {
    int left_char_cost = i;
    int above_char_cost = i;
    const char char_1 = string_1[start + i];
    for (int j = 0; j < len_2; ++j) {
      current_cost = left_char_cost;
      left_char_cost = char_1_costs[j];
      if (string_2[start + j] != char_1) {
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (left_char_cost < current_cost) {
          current_cost = left_char_cost;
        }
        ++current_cost;
      }
      char_1_costs[j] = current_cost;
      above_char_cost = current_cost;
    }
  }
  return current_cost;
}

int _distance_max(
    const std::string string_1
  , const std::string string_2
  , const int len_1
  , const int len_2
  , const int start
  , const int max_distance)
{
  std::vector<int> char_1_costs(len_2);
  for (int i = 0; i < len_2; ++i) {
    char_1_costs[i] = i < max_distance ? i + 1 : max_distance + 1;
  }
  const int len_diff = len_2 - len_1;
  const int j_start_offset = max_distance - len_diff;
  int j_start = 0;
  int j_end = max_distance;
  int current_cost = 0;
  for (int i = 0; i < len_1; ++i) {
    char char_1 = string_1[start + i];
    int prev_char_1_cost = i;
    int above_char_cost = i;
    if (i > j_start_offset) {
      ++j_start;
    }
    if (j_end < len_2) {
      ++j_end;
    }
    for (int j = j_start; j < j_end; ++j) {
      current_cost = prev_char_1_cost;
      prev_char_1_cost = char_1_costs[j];
      if (string_2[start + j] != char_1) {
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (prev_char_1_cost < current_cost) {
          current_cost = prev_char_1_cost;
        }
        ++current_cost;
      }
      char_1_costs[j] = current_cost;
      above_char_cost = current_cost;
    }
    if (char_1_costs[i + len_diff] > max_distance) {
      return -1;
    }
  }
  return current_cost <= max_distance ? current_cost : -1;
}
