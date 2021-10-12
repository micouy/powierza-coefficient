#[inline(always)]
fn is_nonempty(score: u32) -> bool {
    score > 0
}

pub fn powierża_coefficient(pattern: &str, sequence: &str) -> Option<u32> {
    let sequence_len = sequence.chars().count();
    let pattern_len = pattern.chars().count();

    // No need to check sequence_len == 0 separately.
    if pattern_len > sequence_len || pattern_len == 0 {
        return None;
    }

    let mut cache = vec![0; sequence_len];
    let mut left_score = 0;

    let mut is_left_continuation = false;
    let mut does_row_contain_match = false;
    let mut first_match_ix = 0;

    // First row.
    let pattern_first_c = pattern.chars().next().expect("unreachable");

    for (x, sequence_c) in sequence.chars().enumerate() {
        if pattern_first_c == sequence_c {
            cache[x] = 1;
            if !does_row_contain_match {
                first_match_ix = x;
            }

            is_left_continuation = true;
            does_row_contain_match = true;
        } else {
            let left_score = if x == 0 { 0 } else { cache[x - 1] };

            if is_nonempty(left_score) {
                cache[x] = if is_left_continuation {
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

    // The rest.
    for pattern_c in pattern.chars().skip(1) {
        does_row_contain_match = false;
        cache[sequence_len - 1] = left_score;
        left_score = 0;

        for (x, sequence_c) in sequence.chars().enumerate().skip(first_match_ix + 1) {
            // x is guaranteed to be at least 1.
            let left_next_score = if is_left_continuation {
                left_score + 1
            } else {
                left_score
            };

            let current_score = if pattern_c == sequence_c {
                if !does_row_contain_match {
                    first_match_ix = x;
                }
                does_row_contain_match = true;

                let upper_left_score = cache[x - 1];
                let upper_left_next_score = upper_left_score;

                if is_nonempty(left_score) && left_next_score <= upper_left_next_score {
                    is_left_continuation = false;

                    left_next_score
                } else if is_nonempty(upper_left_score) {
                    is_left_continuation = true;

                    upper_left_next_score
                } else {
                    0
                }
            } else if is_nonempty(left_score) {
                is_left_continuation = false;

                left_next_score
            } else {
                0
            };

            cache[x - 1] = left_score;
            left_score = current_score;
        }

        if !does_row_contain_match {
            return None;
        }
    }

    cache[sequence_len - 1] = left_score;

    cache
        .iter()
        .skip(first_match_ix)
        .filter(|score| **score > 0)
        .min()
        .map(|score| score - 1)
}

#[cfg(test)]
mod test {
    use super::powierża_coefficient as powierża;

    #[test]
    fn test_powierża_coefficient() {
        let pattern = "abcjkl";

        assert!(powierża(pattern, "").is_none());
        assert!(powierża(pattern, "xyz").is_none());

        assert_eq!(powierża(pattern, "abcjkl").unwrap(), 0);
        assert_eq!(powierża(pattern, "abc_jkl").unwrap(), 1);
        assert_eq!(powierża(pattern, "a_bcjkl").unwrap(), 1);
        assert_eq!(powierża(pattern, "abc_jk_abcj_l").unwrap(), 2);
        assert_eq!(powierża(pattern, "a_b_c_jkl_ab_c_jkl").unwrap(), 2);
        assert_eq!(powierża(pattern, "a_b_c_abc_j_k_l_jkl").unwrap(), 1);
    }
}
