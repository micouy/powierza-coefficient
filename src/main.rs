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

        for (y, row) in string_matrix.inner.iter().enumerate() {
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

#[derive(Clone, Copy, Debug)]
enum Cell {
    Break(u32),
    Continuation(u32),
}

impl Cell {
    fn next_horizontal(&self) -> Cell {
        match self {
            Cell::Break(score) => Cell::Break(*score),
            Cell::Continuation(score) => Cell::Break(score + 1),
        }
    }

    fn next_diagonal(&self) -> Cell {
        match self {
            Cell::Break(score) => Cell::Continuation(*score),
            Cell::Continuation(score) => Cell::Continuation(*score),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Cell::Break(score) => *score,
            Cell::Continuation(score) => *score,
        }
    }
}

fn next_matching(horizontal: Cell, diagonal: Cell) -> Cell {
    let horizontal_score = horizontal.score();
    let diagonal_score = diagonal.score();

    if diagonal_score <= horizontal_score {
        Cell::Continuation(diagonal_score)
    } else {
        Cell::Break(horizontal_score)
    }
}

fn next_different(horizontal: Cell) -> Cell {
    match horizontal {
        Cell::Continuation(score) => Cell::Break(score + 1),
        Cell::Break(score) => Cell::Break(score),
    }
}

fn main() {
    let pattern = "abcjkl";
    let sequence = "abc_jk_abcj_l";
    let n_rows = pattern.len();
    let n_cols = sequence.len();

    let mut edyta = DumbMatrix::<Cell>::optional(n_cols, n_rows);

    for (y, pattern_letter) in pattern.chars().enumerate() {
        for (x, sequence_letter) in sequence.chars().enumerate() {
            let left = x.checked_sub(1).and_then(|x| edyta.get(x, y)).copied().flatten();
            let upper_left = x
                .checked_sub(1)
                .and_then(|x| y.checked_sub(1).map(|y| (x, y)))
                .and_then(|(x, y)| edyta.get(x, y))
                .copied()
                .flatten();

            let cell = if y == 0 {
                if pattern_letter == sequence_letter {
                    Some(Cell::Continuation(0))
                } else {
                    None
                }
            } else {
                if pattern_letter == sequence_letter {
                    match (left, upper_left) {
                        (Some(left), Some(upper_left)) =>
                            Some(next_matching(left, upper_left)),
                        (None, Some(upper_left)) =>
                            Some(Cell::Continuation(upper_left.score())),
                        (Some(left), None) => Some(Cell::Break(left.score() + 1)),
                        (None, None) => None,
                    }
                } else {
                    if let Some(left) = left {
                        Some(next_different(left))
                    } else {
                        None
                    }
                }

            };

            edyta.set(cell, x, y);
        }
    }

	println!("{} vs {}\n", pattern, sequence);
    println!("{}", edyta);
}
