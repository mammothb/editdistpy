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

#include "_levenshtein.h"

#include <algorithm>
#include <cstdlib>
#include <iostream>

#include "_def.h"
#include "_helpers.h"

/**
 * Matrix of all the possible edit sequences.
 *
 * 01 = DELETE, 10 = INSERT, 11 = REPLACE
 */
static constexpr uint8_t kOpsMatrix[9][8] = {
    // maxDistance=1
    {0x03}, // lenDiff=0
    {0x01}, // lenDiff=1
    // maxDistance=2
    {0x0f, 0x09, 0x06}, // lenDiff=0
    {0x0d, 0x07},       // lenDiff=1
    {0x05},             // lenDiff=2
    // maxDistance=3
    {0x3f, 0x27, 0x2d, 0x39, 0x36, 0x1e, 0x1b}, // lenDiff=0
    {0x3d, 0x37, 0x1f, 0x25, 0x19, 0x16},       // lenDiff=1
    {0x35, 0x1d, 0x17},                         // lenDiff=2
    {0x15}                                      // lenDiff=3
};

int Distance(const int *pString1, const int *pString2, int stringLen1,
             int stringLen2, const int64_t maxDistance) {
  if (pString1 == nullptr || pString2 == nullptr) {
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
  if (maxDistance < 4) {
    return Fujimoto2018(pString2, pString1, stringLen2, stringLen1, start,
                        maxDistance);
  }
  if (maxDistance < stringLen2) {
    return InternalDistanceMax(pString1, pString2, stringLen1, stringLen2,
                               start, maxDistance);
  }
  return InternalDistance(pString1, pString2, stringLen1, stringLen2, start);
}

int Fujimoto2018(const int *pString1, const int *pString2, const int len1,
                 const int len2, const int start, const int64_t maxDistance) {
  // Expects pString1 to be longer than pString2
  const int len_diff = len1 - len2;
  const uint8_t *p_possible_ops =
      kOpsMatrix[(maxDistance + maxDistance * maxDistance) / 2 + len_diff - 1];
  int64_t cost = maxDistance + 1;
  for (int i = 0; p_possible_ops[i] != 0; ++i) {
    uint8_t ops = p_possible_ops[i];
    int s1_pos = 0;
    int s2_pos = 0;
    int64_t curr_cost = 0;
    while (s1_pos < len1 && s2_pos < len2) {
      if (pString1[s1_pos + start] != pString2[s2_pos + start]) {
        ++curr_cost;
        if (!ops) {
          break;
        }
        if (ops & 1) {
          ++s1_pos;
        }
        if (ops & 2) {
          ++s2_pos;
        }
        ops >>= 2;
      } else {
        ++s1_pos;
        ++s2_pos;
      }
    }
    curr_cost += len1 - s1_pos + len2 - s2_pos;
    cost = curr_cost < cost ? curr_cost : cost;
  }
  return cost <= maxDistance ? cost : -1;
}

int InternalDistance(const int *pString1, const int *pString2, const int len1,
                     const int len2, const int start) {
  int *p_char_1_costs = (int *)malloc(len2 * sizeof(int));
  for (int i = 0; i < len2; ++i) {
    p_char_1_costs[i] = i + 1;
  }
  int current_cost = 0;
  for (int i = 0; i < len1; ++i) {
    int above_char_cost = i;
    int left_char_cost = i;
    const int char_1 = pString1[start + i];
    for (int j = 0; j < len2; ++j) {
      current_cost = left_char_cost;
      left_char_cost = p_char_1_costs[j];
      if (char_1 != pString2[start + j]) {
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (left_char_cost < current_cost) {
          current_cost = left_char_cost;
        }
        ++current_cost;
      }
      above_char_cost = current_cost;
      p_char_1_costs[j] = current_cost;
    }
  }
  free(p_char_1_costs);
  return current_cost;
}

int InternalDistanceMax(const int *pString1, const int *pString2,
                        const int len1, const int len2, const int start,
                        const int64_t maxDistance) {
  int *p_char_1_costs = (int *)malloc(len2 * sizeof(int));
  for (int i = 0; i < len2; ++i) {
    p_char_1_costs[i] = i < maxDistance ? i + 1 : maxDistance + 1;
  }
  const int len_diff = len2 - len1;
  const int64_t j_start_offset = maxDistance - len_diff;
  int j_start = 0;
  int64_t j_end = maxDistance;
  int current_cost = 0;
  for (int i = 0; i < len1; ++i) {
    int char_1 = pString1[start + i];
    int above_char_cost = i;
    int prev_char_1_cost = i;
    if (i > j_start_offset) {
      ++j_start;
    }
    if (j_end < len2) {
      ++j_end;
    }
    for (int j = j_start; j < j_end; ++j) {
      current_cost = prev_char_1_cost;
      prev_char_1_cost = p_char_1_costs[j];
      if (char_1 != pString2[start + j]) {
        if (above_char_cost < current_cost) {
          current_cost = above_char_cost;
        }
        if (prev_char_1_cost < current_cost) {
          current_cost = prev_char_1_cost;
        }
        ++current_cost;
      }
      above_char_cost = current_cost;
      p_char_1_costs[j] = current_cost;
    }
    if (p_char_1_costs[i + len_diff] > maxDistance) {
      free(p_char_1_costs);
      return -1;
    }
  }
  free(p_char_1_costs);
  return current_cost <= maxDistance ? current_cost : -1;
}
