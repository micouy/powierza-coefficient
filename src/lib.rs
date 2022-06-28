#![doc = include_str!("../README.md")]

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

    // To save memory, only one "row" of cells is stored at any given time.
    // Note that it contains cells from two rows, because it is filled
    // sequentially - after the last cell of the previous row was filled,
    // the algorithm begins filling the next row from the left.

    // The first row is special, because there is nothing in the diagonal
    // (upper-left) cells. It is padded with `Some(0)` instead of `None` to
    // prevent premature termination.
    let mut cache = vec![Some(0); text_len];

    // The score of the cell on the left. It is cached, because otherwise it
    // would overwrite the diagonal cell. After the left and the diagonal
    // cell are compared, the diagonal one is overwritten.
    let mut left = None;
    let mut is_left_continuation = false;

    // If no match was found in the whole row, the algorithm terminates.
    let mut does_row_contain_match;

    // Index of the first match in the current row.
    let mut first_match_ix = 0;

    for pattern_c in pattern.chars() {
        does_row_contain_match = false;
        left = None;

        // `Some(0)` set only for x = 0. The result — the pattern can begin at
        // any place in the string.
        let mut diagonal = Some(0);

        for (x, text_c) in text.chars().enumerate().skip(first_match_ix) {
            let is_match = pattern_c == text_c;

            let (current_score, is_continuation) =
                calc_score(is_match, diagonal, left, is_left_continuation);

            if is_continuation {
                if !does_row_contain_match {
                    first_match_ix = x;
                }

                does_row_contain_match = true;
            }

            if x > 0 {
                cache[x - 1] = left;
            }

            left = current_score;
            is_left_continuation = is_continuation;
            diagonal = cache[x];
        }

        if !does_row_contain_match {
            return None;
        }
    }

    // Fill the score of the right-most cell in the last row.
    cache[text_len - 1] = left;

    // The lowest score in the last row is the coefficient.
    cache
        .into_iter()
        .skip(first_match_ix)
        .filter_map(|cell| cell)
        .min()
}

#[cfg(test)]
mod test {
    use super::powierża_coefficient as powierża;

    #[test]
    fn test_empty() {
        assert!(powierża("", "").is_none());
    }

    #[test]
    fn test_pattern_longer_than_text() {
        assert!(powierża("abc", "").is_none());
    }

    #[test]
    fn test_no_match() {
        assert!(powierża("abc", "xyz").is_none());
    }

    #[test]
    fn test_match_position() {
        assert_eq!(powierża("abc", "abc").unwrap(), 0);
        assert_eq!(powierża("abc", "abc__").unwrap(), 0);
        assert_eq!(powierża("abc", "_abc_").unwrap(), 0);
        assert_eq!(powierża("abc", "__abc").unwrap(), 0);
    }

    #[test]
    fn test_abcd() {
        let pattern = "abcd";

        assert_eq!(powierża(pattern, "abc_a_b_c_d").unwrap(), 1);
        assert_eq!(powierża(pattern, "a_b_c_d_bcd").unwrap(), 1);
    }

    #[test]
    fn test_abcjkl() {
        let pattern = "abcjkl";

        assert_eq!(powierża(pattern, "abcjkl").unwrap(), 0);
        assert_eq!(powierża(pattern, "abc_jkl").unwrap(), 1);
        assert_eq!(powierża(pattern, "a_bcjkl").unwrap(), 1);
        assert_eq!(powierża(pattern, "abc_jk_abcj_l").unwrap(), 2);
        assert_eq!(powierża(pattern, "a_b_c_jkl_ab_c_jkl").unwrap(), 2);
        assert_eq!(powierża(pattern, "a_b_c_abc_j_k_l_jkl").unwrap(), 1);
    }

    #[test]
    fn test_real_folders() {
        let pattern = "de";

        assert_eq!(powierża(pattern, ".docker").unwrap(), 1);
        assert_eq!(powierża(pattern, "documents").unwrap(), 1);
        assert_eq!(powierża(pattern, "vb-shared-files").unwrap(), 1);
    }
}
