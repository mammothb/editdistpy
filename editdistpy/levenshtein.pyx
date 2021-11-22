cpdef int distance(str string_1, str string_2, int len_1, int len_2, int start):
    char_1_costs = [j + 1 for j in range(len_2)]
    current_cost = 0
    for i in range(len_1):
        left_char_cost = above_char_cost = i
        char_1 = string_1[start + i]
        for j in range(len_2):
            # cost of diagonal (substitution)
            current_cost = left_char_cost
            left_char_cost = char_1_costs[j]
            if string_2[start + j] != char_1:
                # substitution if neither of the two conditions below
                if above_char_cost < current_cost:
                    current_cost = above_char_cost
                if left_char_cost < current_cost:
                    current_cost = left_char_cost
                current_cost += 1
            char_1_costs[j] = above_char_cost = current_cost
    return current_cost

cpdef int distance_max(
    str string_1, str string_2, int len_1, int len_2, int start, int max_distance
):
    char_1_costs = [
        j + 1 if j < max_distance else max_distance + 1 for j in range(len_2)
    ]
    len_diff = len_2 - len_1
    j_start_offset = max_distance - len_diff
    j_start = 0
    j_end = max_distance
    current_cost = 0
    for i in range(len_1):
        char_1 = string_1[start + i]
        prev_char_1_cost = above_char_cost = i
        # no need to look beyond window of lower right diagonal -
        # max_distance cells (lower right diag is i - lenDiff) and the upper
        # left diagonal + max_distance cells (upper left is i)
        j_start += 1 if i > j_start_offset else 0
        j_end += 1 if j_end < len_2 else 0
        for j in range(j_start, j_end):
            # cost of diagonal (substitution)
            current_cost = prev_char_1_cost
            prev_char_1_cost = char_1_costs[j]
            if string_2[start + j] != char_1:
                # substitution if neither of the two conditions below
                if above_char_cost < current_cost:
                    current_cost = above_char_cost
                if prev_char_1_cost < current_cost:
                    current_cost = prev_char_1_cost
                current_cost += 1
            char_1_costs[j] = above_char_cost = current_cost
        if char_1_costs[i + len_diff] > max_distance:
            return -1
    return current_cost if current_cost <= max_distance else -1