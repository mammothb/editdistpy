#include "_damerau_osa.hpp"

int _distance(
    const char* string_1
  , const char* string_2
  , const int len_1
  , const int len_2
  , const int start)
{
  int prev_char_1_costs[len_2] = {};
  int char_1_costs[len_2];
  for (int i = 0; i < len_2; ++i) {
    char_1_costs[i] = i + 1;
  }
  char char_1 = ' ';
  int current_cost = 0;
  for (int i = 0; i < len_1; ++i) {
    const char prev_char_1 = char_1;
    char_1 = string_1[start + i];
    char char_2 = ' ';
    int above_char_cost = i;
    int left_char_cost = i;
    int next_trans_cost = 0;
    for (int j = 0; j < len_2; ++j) {
      const int this_trans_cost = next_trans_cost;
      next_trans_cost = prev_char_1_costs[j];
      // cost of diagonal (substitution)
      current_cost = left_char_cost;
      prev_char_1_costs[j] = left_char_cost;
      // left now equals current cost (which will be diagonal at next
      // iteration)
      left_char_cost = char_1_costs[j];
      const char prev_char_2 = char_2;
      char_2 = string_2[start + j];
      if (char_1 != char_2) {
        // substitution if neither of two conditions below
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (left_char_cost < current_cost) {
          current_cost = left_char_cost;
        }
        ++current_cost;
        if (i != 0
            && j != 0
            && char_1 == prev_char_2
            && prev_char_1 == char_2
            && this_trans_cost + 1 < current_cost) {
          // transposition
          current_cost = this_trans_cost + 1;
        }
      }
      above_char_cost = current_cost;
      char_1_costs[j] = current_cost;
    }
  }
  return current_cost;
}

int _distance_max(
    const char* string_1
  , const char* string_2
  , const int len_1
  , const int len_2
  , const int start
  , const int max_distance)
{
  int prev_char_1_costs[len_2] = {};
  int char_1_costs[len_2];
  for (int i = 0; i < len_2; ++i) {
    char_1_costs[i] = i < max_distance ? i + 1 : max_distance + 1;
  }
  const int len_diff = len_2 - len_1;
  const int j_start_offset = max_distance - len_diff;
  int j_start = 0;
  int j_end = max_distance;
  char char_1 = ' ';
  int current_cost = 0;
  for (int i = 0; i < len_1; ++i) {
    const char prev_char_1 = char_1;
    char_1 = string_1[start + i];
    char char_2 = ' ';
    int above_char_cost = i;
    int left_char_cost = i;
    int next_trans_cost = 0;
    // no need to look beyond window of lower right diagonal - max_distance
    // cells (lower right diag is i - len_diff) and the upper left diagonal
    // + max_distance cells (upper left is i)
    if (i > j_start_offset) {
      ++j_start;
    }
    if (j_end < len_2) {
      ++j_end;
    }
    for (int j = j_start; j < j_end; ++j) {
      const int this_trans_cost = next_trans_cost;
      next_trans_cost = prev_char_1_costs[j];
      // cost of diagonal (substitution)
      current_cost = left_char_cost;
      prev_char_1_costs[j] = left_char_cost;
      // left not equals current cost (which will be diagonal at next
      // iteration)
      left_char_cost = char_1_costs[j];
      const char prev_char_2 = char_2;
      char_2 = string_2[start + j];
      if (char_1 != char_2) {
        // substitution if neither of two conditions below
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (left_char_cost < current_cost) {
          current_cost = left_char_cost;
        }
        ++current_cost;
        if (i != 0
            && j != 0
            && char_1 == prev_char_2
            && prev_char_1 == char_2
            && this_trans_cost + 1 < current_cost) {
          // transposition
          current_cost = this_trans_cost + 1;
        }
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
