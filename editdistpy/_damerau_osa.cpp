/**
 * MIT License
 *
 * Copyright (c) 2024 mmb L
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

#include "_damerau_osa.h"

#include <algorithm>
#include <cstdlib>

#include "_helpers.h"

const int kSpace = 32;

int Distance(const int *pString1, const int *pString2, int stringLen1,
             int stringLen2, const int64_t maxDistance) {
  if (pString1 == NULL || pString2 == NULL) {
    return NullDistanceResults(pString1, pString2, stringLen1, stringLen2,
                               maxDistance);
  }
  if (maxDistance <= 0) {
    return ZeroDistanceResults(pString1, pString2, stringLen1, stringLen2);
  }
  if (stringLen2 - stringLen1 > maxDistance ||
      stringLen2 - stringLen1 < -maxDistance) {
    return -1;
  }
  if (stringLen1 > stringLen2) {
    std::swap(pString1, pString2);
    std::swap(stringLen1, stringLen2);
  }
  // Trim suffix
  while (stringLen1 != 0 && pString1[~-stringLen1] == pString2[~-stringLen2]) {
    --stringLen1;
    --stringLen2;
  }
  if (stringLen1 == 0) {
    return stringLen2 <= maxDistance ? stringLen2 : -1;
  }
  // Trim prefix
  int start = 0;
  while (start != stringLen1 && pString1[start] == pString2[start]) {
    ++start;
  }
  if (start != 0) {
    stringLen1 -= start;
    stringLen2 -= start;
  }
  if (stringLen1 == 0) {
    return stringLen2 <= maxDistance ? stringLen2 : -1;
  }
  if (maxDistance < stringLen2) {
    return InternalDistanceMax(pString1, pString2, stringLen1, stringLen2,
                               start, maxDistance);
  }
  return InternalDistance(pString1, pString2, stringLen1, stringLen2, start);
}

int InternalDistance(const int *pString1, const int *pString2, const int len1,
                     const int len2, const int start) {
  int *p_prev_char_1_costs = (int *)calloc(len2, sizeof(int));
  int *p_char_1_costs = (int *)malloc(len2 * sizeof(int));
  for (int i = 0; i < len2; ++i) {
    p_char_1_costs[i] = i + 1;
  }
  int char_1 = kSpace;
  int current_cost = 0;
  for (int i = 0; i < len1; ++i) {
    const int prev_char_1 = char_1;
    char_1 = pString1[start + i];
    int char_2 = kSpace;
    int above_char_cost = i;
    int left_char_cost = i;
    int next_trans_cost = 0;
    for (int j = 0; j < len2; ++j) {
      const int this_trans_cost = next_trans_cost;
      next_trans_cost = p_prev_char_1_costs[j];
      // cost of diagonal (substitution)
      current_cost = left_char_cost;
      p_prev_char_1_costs[j] = left_char_cost;
      // left now equals current cost (which will be diagonal at next
      // iteration)
      left_char_cost = p_char_1_costs[j];
      const int prev_char_2 = char_2;
      char_2 = pString2[start + j];
      if (char_1 != char_2) {
        // substitution if neither of two conditions below
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (left_char_cost < current_cost) {
          current_cost = left_char_cost;
        }
        ++current_cost;
        if (i != 0 && j != 0 && char_1 == prev_char_2 &&
            prev_char_1 == char_2 && this_trans_cost + 1 < current_cost) {
          // transposition
          current_cost = this_trans_cost + 1;
        }
      }
      above_char_cost = current_cost;
      p_char_1_costs[j] = current_cost;
    }
  }
  free(p_prev_char_1_costs);
  free(p_char_1_costs);
  return current_cost;
}

int InternalDistanceMax(const int *pString1, const int *pString2,
                        const int len1, const int len2, const int start,
                        const int64_t maxDistance) {
  int *p_prev_char_1_costs = (int *)calloc(len2, sizeof(int));
  int *p_char_1_costs = (int *)malloc(len2 * sizeof(int));
  for (int i = 0; i < len2; ++i) {
    p_char_1_costs[i] = i + 1;
  }
  const int len_diff = len2 - len1;
  const int64_t j_start_offset = maxDistance - len_diff;
  int j_start = 0;
  int64_t j_end = maxDistance;
  int char_1 = kSpace;
  int current_cost = 0;
  for (int i = 0; i < len1; ++i) {
    const int prev_char_1 = char_1;
    char_1 = pString1[start + i];
    int char_2 = kSpace;
    int above_char_cost = i;
    int left_char_cost = i;
    int next_trans_cost = 0;
    // no need to look beyond window of lower right diagonal - maxDistance
    // cells (lower right diag is i - len_diff) and the upper left diagonal
    // + maxDistance cells (upper left is i)
    if (i > j_start_offset) {
      ++j_start;
    }
    if (j_end < len2) {
      ++j_end;
    }
    for (int j = j_start; j < j_end; ++j) {
      const int this_trans_cost = next_trans_cost;
      next_trans_cost = p_prev_char_1_costs[j];
      // cost of diagonal (substitution)
      current_cost = left_char_cost;
      p_prev_char_1_costs[j] = left_char_cost;
      // left not equals current cost (which will be diagonal at next
      // iteration)
      left_char_cost = p_char_1_costs[j];
      const int prev_char_2 = char_2;
      char_2 = pString2[start + j];
      if (char_1 != char_2) {
        // substitution if neither of two conditions below
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (left_char_cost < current_cost) {
          current_cost = left_char_cost;
        }
        ++current_cost;
        if (i != 0 && j != 0 && char_1 == prev_char_2 &&
            prev_char_1 == char_2 && this_trans_cost + 1 < current_cost) {
          // transposition
          current_cost = this_trans_cost + 1;
        }
      }
      above_char_cost = current_cost;
      p_char_1_costs[j] = current_cost;
    }
    if (p_char_1_costs[i + len_diff] > maxDistance) {
      free(p_prev_char_1_costs);
      free(p_char_1_costs);
      return -1;
    }
  }
  free(p_prev_char_1_costs);
  free(p_char_1_costs);
  return current_cost <= maxDistance ? current_cost : -1;
}
