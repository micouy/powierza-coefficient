#![doc = include_str!("../README.md")]

/// Computes Powierża coefficient of a pattern and a piece of text. For a
/// detailed explanation, see [lib's docs](crate).
pub fn powierża_coefficient(pattern: &str, text: &str) -> Option<u32> {
    let text_len = text.chars().count();
    let pattern_len = pattern.chars().count();

    // The coefficient is undefined if either of the three is true:
    // 1. The pattern is longer than the text.
    // 2. The pattern length is 0.
    // 3. The text length is 0. (This check is unnecessary but makes the
    //    code more understandable.)
    if pattern_len > text_len || pattern_len == 0 || text_len == 0 {
        return None;
    }

    // Pad the matrix with 0s on the top.
    let mut cache = vec![Some(0); text_len];

    let mut left = None;
    let mut is_left_continuation = false;
    let mut first_match_in_row_ix = None;

    for pattern_char in pattern.chars() {
        left = None;

        // Pad the matrix with 0s on the left.
        let mut diagonal = Some(0);

        let text_chars = text.chars().enumerate().skip(first_match_in_row_ix.unwrap_or(0));

        for (x, text_char) in text_chars {
            let is_match = pattern_char == text_char;

            let (current_score, is_continuation) =
                calc_score(is_match, diagonal, left, is_left_continuation);

            if is_continuation {
                first_match_in_row_ix = first_match_in_row_ix.or(Some(x))
            }

            if x > 0 {
                cache[x - 1] = left;
            }

            left = current_score;
            is_left_continuation = is_continuation;
            diagonal = cache[x];
        }

        // Terminate if no match was found in the current row.
        if first_match_in_row_ix.is_none() {
            return None;
        }
    }

    // Fill the score of the right-most cell in the last row.
    cache[text_len - 1] = left;

    // The lowest score in the last row is the coefficient.
    cache
        .into_iter()
        .skip(first_match_ix.unwrap_or(0))
        .filter_map(|cell| cell)
        .min()
}

fn calc_mismatch_score(
    left: Option<u32>,
    is_left_continuation: bool,
) -> Option<u32> {
    left.map(|left_score| {
        if is_left_continuation {
            left_score + 1
        } else {
            left_score
        }
    })
}

fn calc_match_score(
    diagonal: Option<u32>,
    left: Option<u32>,
    is_left_continuation: bool,
) -> (Option<u32>, bool) {
    let gap_score = calc_mismatch_score(left, is_left_continuation);
    let continuation_score = diagonal;

    let (current_score, is_continuation) = match (gap_score, continuation_score)
    {
        (None, Some(continuation_score)) => {
            // If only diagonal contains score, return Continuation.
            (Some(continuation_score), true)
        }
        (Some(gap_score), None) => {
            // If only left contains score, return Gap.
            //
            // This branch is probably unreachable.
            (Some(gap_score), false)
        }
        (Some(gap_score), Some(continuation_score)) => {
            // Choose gap if it has a lower or equal score to continuation.
            if gap_score <= continuation_score {
                (Some(gap_score), false)
            } else {
                (Some(continuation_score), true)
            }
        }
        (None, None) => {
            // If neither left nor diagonal contains a score, leave the cell
            // empty.
            (None, false)
        }
    };

    (current_score, is_continuation)
}

fn calc_score(
    is_match: bool,
    diagonal: Option<u32>,
    left: Option<u32>,
    is_left_continuation: bool,
) -> (Option<u32>, bool) {
    if is_match {
        calc_match_score(diagonal, left, is_left_continuation)
    } else {
        let current_score = calc_mismatch_score(left, is_left_continuation);
        let is_continuation = false;

        (current_score, is_continuation)
    }
}


#[cfg(test)]
mod test {
    use super::powierża_coefficient as powierża;

    #[test]
    fn test_coefficient_should_be_undefined_if_pattern_is_empty() {
        assert!(powierża("", "text is not empty").is_none());
    }

    #[test]
    fn test_coefficient_should_be_undefined_if_text_is_empty() {
        assert!(powierża("pattern is not empty", "").is_none());
    }

    #[test]
    fn test_coefficient_should_be_undefined_if_text_is_longer_than_tet() {
        assert!(powierża(
            "pattern is not empty______________________________________",
            "text is not empty but it is shorter than the pattern"
        ).is_none());
    }

    #[test]
    fn test_coefficient_should_be_undefined_if_pattern_is_not_present_in_text() {
        assert!(powierża("abc", "xyz").is_none());
    }

    #[test]
    fn test_coefficient_value_with_progressing_number_of_gaps() {
        let pattern = "xddd";
        assert_eq!(powierża(pattern, "xddd").unwrap(), 0);
        assert_eq!(powierża(pattern, "xdd_d").unwrap(), 1);
        assert_eq!(powierża(pattern, "xd_d_d").unwrap(), 2);
        assert_eq!(powierża(pattern, "x_d_d_d").unwrap(), 3);
    }

    #[test]
    fn test_coefficient_value_should_not_change_when_part_with_higher_value_is_added() {
        let pattern = "xddd";

        // The prefix `x_d_d_d` should not influence the coefficient
        // because matching it would result in higher (worse) score.

        assert_eq!(powierża(pattern, "x_d_d_d___xddd").unwrap(), 0);
        assert_eq!(powierża(pattern, "x_d_d_d___xdd_d").unwrap(), 1);
        assert_eq!(powierża(pattern, "x_d_d_d___xd_d_d").unwrap(), 2);
        assert_eq!(powierża(pattern, "x_d_d_d___x_d_d_d").unwrap(), 3);
    }

    #[test]
    fn test_coefficient_value_should_not_change_when_gap_length_changes() {
        let pattern = "xddd";

        assert_eq!(powierża(pattern, "x_d_d_d").unwrap(), 3);
        assert_eq!(powierża(pattern, "x_d_d__d").unwrap(), 3);
        assert_eq!(powierża(pattern, "x_d__d__d").unwrap(), 3);
        assert_eq!(powierża(pattern, "x__d__d__d").unwrap(), 3);
        assert_eq!(powierża(pattern, "x__d__d__d").unwrap(), 3);
        assert_eq!(powierża(pattern, "x__d__d__d").unwrap(), 3);
    }

    #[test]
    fn test_coefficient_value_should_not_change_when_gap_is_moved() {
        let pattern = "xddd";

        assert_eq!(powierża(pattern, "xdd_d").unwrap(), 1);
        assert_eq!(powierża(pattern, "xd_dd").unwrap(), 1);
        assert_eq!(powierża(pattern, "x_ddd").unwrap(), 1);
    }

    #[test]
    fn test_coefficient_value_should_not_change_when_prefix_is_added() {
        let pattern = "xddd";

        assert_eq!(powierża(pattern, "xddd_").unwrap(), 0);
        assert_eq!(powierża(pattern, "xdd_d_").unwrap(), 1);
        assert_eq!(powierża(pattern, "xd_d_d_").unwrap(), 2);
        assert_eq!(powierża(pattern, "x_d_d_d_").unwrap(), 3);
    }

    #[test]
    fn test_coefficient_value_should_not_change_when_suffix_is_added() {
        let pattern = "xddd";

        assert_eq!(powierża(pattern, "_xddd").unwrap(), 0);
        assert_eq!(powierża(pattern, "_xdd_d").unwrap(), 1);
        assert_eq!(powierża(pattern, "_xd_d_d").unwrap(), 2);
        assert_eq!(powierża(pattern, "_x_d_d_d").unwrap(), 3);
    }

    #[test]
    fn test_coefficient_value_with_real_folder_names() {
        let pattern = "de";

        assert_eq!(powierża(pattern, ".docker").unwrap(), 1);
        assert_eq!(powierża(pattern, "documents").unwrap(), 1);
        assert_eq!(powierża(pattern, "vb-shared-files").unwrap(), 1);
    }
}
