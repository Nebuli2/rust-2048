use matrix::Matrix;

const BOARD_SIZE: (usize, usize) = (4, 4);

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

pub trait Game<S> {

    /// Produces the score for the `Game`.
    fn score(&self) -> S;

    /// Attempts to slide the `Game` in the specified direction, producing a
    /// new `Game` containing the new state of the `Game`.
    fn slide(self, dir: Direction) -> Self;
}

/// Represents a sliding game.
pub struct SlideGame {
    board: Matrix<u32>,
    turns: usize
}

impl Default for SlideGame {

    /// Produces a default `SlideGame`. A default `SlideGame` contains an 4x4
    /// board, wherein each tile has a value of `0`.
    fn default() -> Self {
        let (rows, cols) = BOARD_SIZE;
        Self {
            board: Matrix::new(rows, cols),
            turns: 0
        }
    }
}

impl Game<u32> for SlideGame {

    /// Produces the score of the `SlideGame`. The `SlideGame`'s score is
    /// defined as the sum of its individual tiles.
    fn score(&self) -> u32 {
        self.board.iter().sum()
    }

    /// Attempts to slide the `SlideGame` in the specified `Direction`.
    /// Consumes the `SlideGame` and produces a `SlideGame` with the new state.
    fn slide(self, dir: Direction) -> Self {
        use self::Direction::*;
        let (rows, cols) = BOARD_SIZE;
        let mut buf = Matrix::<u32>::new(rows, cols);
        match dir {
            Left => slide_left(&self.board, &mut buf),
            Right => slide_right(&self.board, &mut buf),
            Up => slide_up(&self.board, &mut buf),
            Down => slide_down(&self.board, &mut buf),
        }
        Self {
            board: buf,
            turns: self.turns + 1
        }
    }
}

/// Slides the elements of the specified source `Matrix` left, saving the
/// results in the specified destination `Matrix`.
fn slide_left(source: &Matrix<u32>, dest: &mut Matrix<u32>) {
    // Slide row by row
    for i in 0..source.rows() {
        let row = source.row(i);
        for j in 0..source.cols() {
            row[1];
        }
    }
}

/// Slides the elements of the specified source `Matrix` right, saving the
/// results in the specified destination `Matrix`.
fn slide_right(source: &Matrix<u32>, dest: &mut Matrix<u32>) {

}

/// Slides the elements of the specified source `Matrix` up, saving the
/// results in the specified destination `Matrix`.
fn slide_up(source: &Matrix<u32>, dest: &mut Matrix<u32>) {

}

/// Slides the elements of the specified source `Matrix` down, saving the
/// results in the specified destination `Matrix`.
fn slide_down(source: &Matrix<u32>, dest: &mut Matrix<u32>) {

}