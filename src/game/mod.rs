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

    fn slide(self, dir: Direction) -> Self;
}

pub struct SlideGame {
    board: Matrix<u32>,
    turns: usize
}

impl Default for SlideGame {
    fn default() -> Self {
        let (rows, cols) = BOARD_SIZE;
        Self {
            board: Matrix::new(rows, cols),
            turns: 0
        }
    }
}

impl Game<u32> for SlideGame {

    fn score(&self) -> u32 {
        self.board.iter().sum()
    }

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

fn slide_left(source: &Matrix<u32>, dest: &mut Matrix<u32>) {
    // Slide row by row
    for i in 0..source.rows() {
        
    }
}

fn slide_right(source: &Matrix<u32>, dest: &mut Matrix<u32>) {

}

fn slide_up(source: &Matrix<u32>, dest: &mut Matrix<u32>) {

}

fn slide_down(source: &Matrix<u32>, dest: &mut Matrix<u32>) {

}