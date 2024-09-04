#include "_levenshtein.h"

#include <algorithm>
#include <cstdlib>

#include "_helpers.h"

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
