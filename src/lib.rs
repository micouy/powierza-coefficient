use std::{
    fmt::{self, Write},
    mem,
};

struct DumbMatrix<T> {
    inner: Vec<Vec<T>>,
    size: (usize, usize),
}

impl<T> DumbMatrix<T> {
    fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let inner = (0..height)
            .map(|_| (0..width).map(|_| T::default()).collect())
            .collect();

        Self {
            inner,
            size: (width, height),
        }
    }

    fn optional(width: usize, height: usize) -> DumbMatrix<Option<T>> {
        DumbMatrix::new(width, height)
    }

    fn fill(value: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        let inner = (0..height)
            .map(|_| (0..width).map(|_| value.clone()).collect())
            .collect();

        Self {
            inner,
            size: (width, height),
        }
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn set(&mut self, value: T, x: usize, y: usize) -> Option<T> {
        self.inner
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
            .and_then(|cell| Some(mem::replace(cell, value)))
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.inner.get(y).and_then(|row| row.get(x))
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.inner.get_mut(y).and_then(|row| row.get_mut(x))
    }
}

impl<T> fmt::Display for DumbMatrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = self.size();
        let mut string_matrix = DumbMatrix::<String>::new(width, height);
        let mut max_width = 0;

        for (y, row) in self.inner.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if let Some(cell) = string_matrix.get_mut(x, y) {
                    write!(cell, "{:?}", value)?;
                    max_width = max_width.max(cell.chars().count());
                }
            }
        }

        for row in string_matrix.inner.iter() {
            for (x, value) in row.iter().enumerate() {
                if x != 0 {
                    write!(f, "\t")?;
                }

                write!(f, "{:>width$}", value, width = max_width)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

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

fn powierża_distance_optional_short_circuiting(
    pattern: &str,
    sequence: &str,
    max_allowed_score: Option<u32>,
) -> Option<u32> {
    let n_rows = pattern.len();
    let n_cols = sequence.len();

    let mut matrix = DumbMatrix::<u32>::optional(n_cols, n_rows);

    fn get_left(
        matrix: &DumbMatrix<Option<u32>>,
        x: usize,
        y: usize,
    ) -> Option<u32> {
        x.checked_sub(1)
            .and_then(|x| matrix.get(x, y))
            .copied()
            .flatten()
    }

    fn get_upper_left(
        matrix: &DumbMatrix<Option<u32>>,
        x: usize,
        y: usize,
    ) -> Option<u32> {
        x.checked_sub(1)
            .and_then(|x| y.checked_sub(1).map(|y| (x, y)))
            .and_then(|(x, y)| matrix.get(x, y))
            .copied()
            .flatten()
    }

    fn get_distance(matrix: &DumbMatrix<Option<u32>>) -> Option<u32> {
        matrix
            .inner
            .last()
            .and_then(|last_row| last_row.iter().flatten().min().copied())
    }

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

            let score = match result {
                Some((score, _)) => {
                    min_row_score = min_row_score
                        .map(|min_row_score: u32| min_row_score.min(score))
                        .or_else(|| Some(score));

                    Some(score)
                }
                None => None,
            };
            is_left_continuation = result
                .map(|(_, is_continuation)| is_continuation)
                .unwrap_or(false);

            matrix.set(score, x, y);
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
