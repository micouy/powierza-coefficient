#![doc = include_str!("../README.md")]

/// Computes Powierża coefficient of a pattern and a piece of text. For a
/// detailed explanation, see [lib's docs](crate).
pub fn powierża_coefficient(pattern: &str, text: &str) -> Option<u32> {
    let text_len = text.chars().count();
    let pattern_len = pattern.chars().count();

    // The coefficient is undefined if either of the three is true:
    // 1. The pattern is longer than the text.
    // 2. The pattern length is 0.
    // 3. The text length is 0.
    //
    // The third check is redundant but makes code more understandable.
    if pattern_len > text_len || pattern_len == 0 || text_len == 0 {
        return None;
    }

    // The matrix is padded with 0s on the top.
    let mut previous_row = vec![Some(0); text_len];
    let mut n_chars_to_skip = 0;

    for (y, pattern_char) in pattern.chars().enumerate() {
        let n_chars_to_omit = pattern_len - y - 1;

        let (current_row, first_match_ix) = match_pattern_char(
            pattern_char,
            text,
            &previous_row,
            n_chars_to_skip,
            n_chars_to_omit,
        )?;

        previous_row.clear();
        previous_row.extend_from_slice(&current_row);
        n_chars_to_skip = first_match_ix + 1;
    }

    let first_match_ix_in_last_row = n_chars_to_skip - 1;

    let coefficient = previous_row
        .into_iter()
        .skip(first_match_ix_in_last_row)
        .filter_map(|cell| cell)
        .min();
    
    coefficient
}

type Row = Vec<Option<u32>>;

fn match_pattern_char(
    pattern_char: char,
    text: &str,
    previous_row: &Row,
    n_chars_to_skip: usize,
    n_chars_to_omit: usize,
) -> Option<(Row, usize)> {
    let text_len = text.chars().count();

    let text_chars = text
        .chars()
        .enumerate()
        .take(text_len - n_chars_to_omit)
        .skip(n_chars_to_skip);

    let mut current_row = vec![None; text_len];
    let mut first_match_ix = None;
    let mut left_score_and_is_gap = None;

    for (x, text_char) in text_chars {
        let is_match = pattern_char == text_char;
        let diagonal_score = get_diagonal_score(&previous_row, x);

        let current_score_and_is_gap = calc_score(is_match, diagonal_score, left_score_and_is_gap);

        left_score_and_is_gap = current_score_and_is_gap;

        match current_score_and_is_gap {
            Some((score, is_gap)) => {
                if !is_gap {
                    first_match_ix = first_match_ix.or(Some(x));
                }

                current_row[x] = Some(score);
            }
            None => {
                current_row[x] = None;
            }
        }
    }

    match first_match_ix {
        None => return None,
        Some(first_match_ix) => return Some((current_row, first_match_ix)),
    }
}

#[inline]
fn get_diagonal_score(previous_row: &Row, x: usize) -> Option<u32> {
    if x > 0 {
        previous_row[x - 1]
    } else {
        Some(0)
    }
}

fn calc_score(
    is_match: bool,
    diagonal_score: Option<u32>,
    left_score_and_is_gap: Option<(u32, bool)>,
) -> Option<(u32, bool)> {
    if is_match {
        calc_match_score(diagonal_score, left_score_and_is_gap)
    } else {
        let score = calc_mismatch_score(left_score_and_is_gap)?;

        Some((score, true))
    }
}

fn calc_match_score(
    diagonal_score: Option<u32>,
    left_score_and_is_gap: Option<(u32, bool)>,
) -> Option<(u32, bool)> {
    let continuation_score = calc_continuation_score(diagonal_score);
    let gap_score = calc_gap_score(left_score_and_is_gap);

    match (continuation_score, gap_score) {
        (None, None) => None,
        (Some(continuation_score), None) => Some((continuation_score, false)),
        (None, Some(_gap_score)) => unreachable!(),
        (Some(continuation_score), Some(gap_score)) => {
            if continuation_score <= gap_score {
                Some((continuation_score, false))
            } else {
                Some((gap_score, true))
            }
        }
    }
}

#[inline]
fn calc_mismatch_score(left_score_and_is_gap: Option<(u32, bool)>) -> Option<u32> {
    calc_gap_score(left_score_and_is_gap)
}

#[inline]
fn calc_continuation_score(diagonal_score: Option<u32>) -> Option<u32> {
    diagonal_score
}

#[inline]
fn calc_gap_score(left_score_and_is_gap: Option<(u32, bool)>) -> Option<u32> {
    let (left_score, is_left_gap) = left_score_and_is_gap?;

    if is_left_gap {
        Some(left_score)
    } else {
        Some(left_score + 1)
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
            " text is not empty but its shorter than pattern"
        )
        .is_none());
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
    fn test_coefficient_value_should_not_change_when_suffix_is_added() {
        let pattern = "xddd";

        assert_eq!(powierża(pattern, "xddd_").unwrap(), 0);
        assert_eq!(powierża(pattern, "xdd_d_").unwrap(), 1);
        assert_eq!(powierża(pattern, "xd_d_d_").unwrap(), 2);
        assert_eq!(powierża(pattern, "x_d_d_d_").unwrap(), 3);
    }

    #[test]
    fn test_coefficient_value_should_not_change_when_prefix_is_added() {
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
