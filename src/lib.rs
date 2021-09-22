use std::mem;

#[inline(always)]
fn is_nonempty(score: u32) -> bool {
    score > 0
}

pub fn powierża_distance(pattern: &str, sequence: &str) -> Option<u32> {
    let sequence_len = sequence.chars().count();
    let pattern_len = pattern.chars().count();

    // No need to check sequence_len == 0 separately.
    if pattern_len > sequence_len || pattern_len == 0 {
        return None;
    }

    let mut vec_1 = vec![0; sequence_len];
    let mut vec_2 = vec![0; sequence_len];

    let mut previous_row: &mut Vec<u32> = &mut vec_1;
    let mut current_row: &mut Vec<u32> = &mut vec_2;

    let mut is_left_continuation = false;
    let mut does_row_contain_match = false;

    // First row.
    let pattern_first_c = pattern.chars().next().expect("unreachable");

    for (x, sequence_c) in sequence.chars().enumerate() {
        if pattern_first_c == sequence_c {
            previous_row[x] = 1;
            is_left_continuation = true;
            does_row_contain_match = true;
        } else {
            let left_score = if x == 0 { 0 } else { previous_row[x - 1] };

            if is_nonempty(left_score) {
                previous_row[x] = if is_left_continuation {
                    left_score + 1
                } else {
                    left_score
                };
            }
        }
    }

    if !does_row_contain_match {
        return None;
    }
    does_row_contain_match = false;

    // The rest.
    for (y, pattern_c) in pattern.chars().enumerate().skip(1) {
        for (x, sequence_c) in sequence.chars().enumerate().skip(y) {
            // x is guaranteed to be at least 1.
            let left_score = current_row[x - 1];
            let left_next_score = if is_left_continuation {
                left_score + 1
            } else {
                left_score
            };

            if pattern_c == sequence_c {
                does_row_contain_match = true;
                let upper_left_score = previous_row[x - 1];
                let upper_left_next_score = upper_left_score;

                if is_nonempty(left_score)
                    && left_next_score <= upper_left_next_score
                {
                    current_row[x] = left_next_score;
                    is_left_continuation = false;
                } else if is_nonempty(upper_left_score) {
                    current_row[x] = upper_left_next_score;
                    is_left_continuation = true;
                } // ...else leave the cell empty.
            } else if is_nonempty(left_score) {
                current_row[x] = left_next_score;
                is_left_continuation = false;
            } // ...else leave the cell empty.
        }

        if !does_row_contain_match {
            return None;
        }
        does_row_contain_match = false;
        mem::swap(&mut current_row, &mut previous_row);
        current_row.fill(0);
    }

    previous_row
        .iter()
        .filter(|score| **score > 0)
        .min()
        .map(|score| score - 1)
}

#[cfg(test)]
mod test {
    use super::powierża_distance as powierża;

    #[test]
    fn testyyy() {
        let pattern = "abcjkl";

        assert_eq!(powierża(pattern, "abcjkl").unwrap(), 0);
        assert_eq!(powierża(pattern, "abc_jkl").unwrap(), 1);
        assert_eq!(powierża(pattern, "a_bcjkl").unwrap(), 1);
        assert_eq!(powierża(pattern, "abc_jk_abcj_l").unwrap(), 2);
        assert_eq!(powierża(pattern, "a_b_c_jkl_ab_c_jkl").unwrap(), 2);
        assert_eq!(powierża(pattern, "a_b_c_abc_j_k_l_jkl").unwrap(), 1);
    }
}
