#[derive(Debug)]
struct ScoreMatrix {
    inner: Vec<Option<u32>>,
    width: usize,
    height: usize,
}

impl ScoreMatrix {
    fn new(width: usize, height: usize) -> Self {
        let inner = (0..(width * height)).map(|_| None).collect();

        Self {
            inner,
            width,
            height,
        }
    }

    #[inline(always)]
    fn set(&mut self, score: u32, x: usize, y: usize) {
        if let Some(prev_score) = self.inner.get_mut(y * self.width + x) {
            *prev_score = Some(score);
        }
    }

    #[inline(always)]
    fn get(&self, x: usize, y: usize) -> Option<&u32> {
        self.inner
            .get(y * self.width + x)
            .and_then(|inner| inner.as_ref())
    }
}

#[inline(always)]
fn get_score(
    left: Option<u32>,
    upper_left: Option<u32>,
    does_match: bool,
    is_left_continuation: bool,
) -> Option<(u32, bool)> {
    match (left, upper_left) {
        (None, None) => None,
        (Some(left_score), None) =>
            if is_left_continuation {
                Some((left_score + 1, false))
            } else {
                Some((left_score, false))
            },
        (None, Some(upper_left_score)) =>
            if does_match {
                Some((upper_left_score, true))
            } else {
                None
            },
        (Some(left_score), Some(upper_left_score)) => {
            let next_score_left = if is_left_continuation {
                left_score + 1
            } else {
                left_score
            };
            let next_score_upper_left = upper_left_score;

            if does_match {
                if next_score_upper_left <= next_score_left {
                    Some((next_score_upper_left, true))
                } else {
                    Some((next_score_left, false))
                }
            } else {
                Some((next_score_left, false))
            }
        }
    }
}

#[inline(always)]
fn get_left(matrix: &ScoreMatrix, x: usize, y: usize) -> Option<u32> {
    x.checked_sub(1).and_then(|x| matrix.get(x, y)).copied()
}

#[inline(always)]
fn get_upper_left(matrix: &ScoreMatrix, x: usize, y: usize) -> Option<u32> {
    x.checked_sub(1)
        .and_then(|x| y.checked_sub(1).map(|y| (x, y)))
        .and_then(|(x, y)| matrix.get(x, y))
        .copied()
}

#[inline(always)]
fn get_distance(matrix: &ScoreMatrix) -> Option<u32> {
    let start = matrix.inner.len() - matrix.width;
    let end = matrix.inner.len();

    matrix.inner[start..end].iter().flatten().min().copied()
}

fn powierża_distance_optional_short_circuiting(
    pattern: &str,
    sequence: &str,
    max_allowed_score: Option<u32>,
) -> Option<u32> {
    let n_rows = pattern.len();
    let n_cols = sequence.len();

    let mut matrix = ScoreMatrix::new(n_cols, n_rows);

    for (y, pattern_letter) in pattern.chars().enumerate() {
        let mut min_row_score = None;
        let mut is_left_continuation = false;

        for (x, sequence_letter) in
            sequence.chars().enumerate().skip(y).take(n_cols - y)
        {
            let left = get_left(&matrix, x, y);
            let upper_left = get_upper_left(&matrix, x, y);

            let result = if y == 0 {
                if pattern_letter == sequence_letter {
                    Some((0, true))
                } else {
                    get_score(left, None, false, is_left_continuation)
                }
            } else {
                let does_match = pattern_letter == sequence_letter;

                get_score(left, upper_left, does_match, is_left_continuation)
            };

            if let Some((score, _)) = result {
                min_row_score = min_row_score
                    .map(|min_row_score: u32| min_row_score.min(score))
                    .or(Some(score));

                matrix.set(score, x, y);
            }

            is_left_continuation = result
                .map(|(_, is_continuation)| is_continuation)
                .unwrap_or(false);
        }

        match (min_row_score, max_allowed_score) {
            (Some(min_row_score), Some(max_allowed_score)) =>
                if min_row_score > max_allowed_score {
                    return None;
                },
            (Some(_), None) => {}
            (None, _) => return None,
        }
    }

    get_distance(&matrix)
}

pub fn powierża_distance(pattern: &str, sequence: &str) -> Option<u32> {
    powierża_distance_optional_short_circuiting(pattern, sequence, None)
}

pub fn powierża_distance_short_circuiting(
    pattern: &str,
    sequence: &str,
    max_allowed_score: u32,
) -> Option<u32> {
    powierża_distance_optional_short_circuiting(
        pattern,
        sequence,
        Some(max_allowed_score),
    )
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
