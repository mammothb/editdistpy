cpdef int distance(str string_1, str string_2, int len_1, int len_2, int start):
    char_1_costs = [j + 1 for j in range(len_2)]
    prev_char_1_costs = [0 for _ in range(len_2)]
    char_1 = " "
    current_cost = 0
    for i in range(len_1):
        prev_char_1 = char_1
        char_1 = string_1[start + i]
        char_2 = " "
        left_char_cost = above_char_cost = i
        next_trans_cost = 0
        for j in range(len_2):
            this_trans_cost = next_trans_cost
            next_trans_cost = prev_char_1_costs[j]
            # cost of diagonal (substitution)
            prev_char_1_costs[j] = current_cost = left_char_cost
            # left now equals current cost (which will be diagonal
            # at next iteration)
            left_char_cost = char_1_costs[j]
            prev_char_2 = char_2
            char_2 = string_2[start + j]
            if char_1 != char_2:
                # substitution if neither of two conditions below
                if above_char_cost < current_cost:
                    current_cost = above_char_cost
                if left_char_cost < current_cost:
                    current_cost = left_char_cost
                current_cost += 1
                if (
                    i != 0
                    and j != 0
                    and char_1 == prev_char_2
                    and prev_char_1 == char_2
                    and this_trans_cost + 1 < current_cost
                ):
                    # transposition
                    current_cost = this_trans_cost + 1
            char_1_costs[j] = above_char_cost = current_cost
    return current_cost

cpdef int distance_max(
    str string_1, str string_2, int len_1, int len_2, int start, int max_distance
):
    char_1_costs = [
        j + 1 if j < max_distance else max_distance + 1 for j in range(len_2)
    ]
    prev_char_1_costs = [0 for _ in range(len_2)]
    len_diff = len_2 - len_1
    j_start_offset = max_distance - len_diff
    j_start = 0
    j_end = max_distance
    char_1 = " "
    current_cost = 0
    for i in range(len_1):
        prev_char_1 = char_1
        char_1 = string_1[start + i]
        char_2 = " "
        left_char_cost = above_char_cost = i
        next_trans_cost = 0
        # no need to look beyond window of lower right diagonal -
        # max_distance cells (lower right diag is i - len_diff) and the upper
        # left diagonal + max_distance cells (upper left is i)
        j_start += 1 if i > j_start_offset else 0
        j_end += 1 if j_end < len_2 else 0
        for j in range(j_start, j_end):
            this_trans_cost = next_trans_cost
            next_trans_cost = prev_char_1_costs[j]
            # cost of diagonal (substitution)
            prev_char_1_costs[j] = current_cost = left_char_cost
            # left now equals current cost (which will be diagonal at next
            # iteration)
            left_char_cost = char_1_costs[j]
            prev_char_2 = char_2
            char_2 = string_2[start + j]
            if char_1 != char_2:
                # substitution if neither of two conditions below
                if above_char_cost < current_cost:
                    current_cost = above_char_cost
                if left_char_cost < current_cost:
                    current_cost = left_char_cost
                current_cost += 1
                if (
                    i != 0
                    and j != 0
                    and char_1 == prev_char_2
                    and prev_char_1 == char_2
                    and this_trans_cost + 1 < current_cost
                ):
                    # transposition
                    current_cost = this_trans_cost + 1
            char_1_costs[j] = above_char_cost = current_cost
        if char_1_costs[i + len_diff] > max_distance:
            return -1
    return current_cost if current_cost <= max_distance else -1
