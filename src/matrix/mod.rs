use std::fmt::{Display, Formatter, Error};
use std::ops::{Index};

#[macro_use]
pub mod macros;

/// Represents a `Matrix` comprised of elements of an arbitrary scalar type
/// `T`.
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize
}

impl<T> Matrix<T> {

    /// Produces the number of rows in the `Matrix`.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Produces the number of columns in the `Matrix`.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Produces a tuple containing the number of rows and columns in the
    /// `Matrix` in the form `(rows, cols)`.
    pub fn size(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    /// Produces a `Vec` index equivalent to the specified row and column
    /// indices.
    fn index(&self, i: usize, j: usize) -> usize {
        i * self.cols() + j
    }

    /// Produces a reference to the element at the specified row and column
    /// indices in the `Matrix`.
    pub fn get(&self, i: usize, j: usize) -> &T {
        let index = self.index(i, j);
        &self.data[index]
    }

    /// Sets the element at the specified row and column indices in the
    /// `Matrix` to the specified value.
    pub fn set(&mut self, i: usize, j: usize, val: T) {
        let index = self.index(i, j);
        self.data[index] = val;
    }

    /// Produces a `Iterator` containing references to each value in the
    /// specified row of the `Matrix`.
    pub fn row(&self, i: usize) -> Row<T> {
        Row {
            matrix: self,
            row: i,
            col_index: 0
        }
    }

    /// Produces a `Iterator` containing references to each value in the
    /// specified column of the `Matrix`.
    pub fn col(&self, j: usize) -> Col<T> {
        Col {
            matrix: self,
            row_index: 0,
            col: j
        }
    }

    /// Produces an `Iterator` containing the references to the diagonal values
    /// of the square `Matrix`.
    ///
    /// ### Precondition
    /// `self` must be a square `Matrix`.
    pub fn diag(&self) -> Diag<T> {
        let (rows, cols) = self.size();
        if rows != cols {
            panic!("Matrix::diag cannot be called on a non-square Matrix");
        }

        Diag {
            matrix: self,
            index: 0
        }
    }

    /// Produces an `Iterator` containing references to each value in the
    /// `Matrix`. Values are iterated over by column, then by row.
    ///
    /// As an example, the values in the below `Matrix` indicate their order in
    /// the iteration:
    /// ```
    /// | 1  2  3 |
    /// | 4  5  6 |
    /// | 7  8  9 |
    /// ```
    pub fn iter(&self) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            index: 0
        }
    }
}

impl<T> Matrix<T> where T: Clone {

    /// Produces the transpose of the `Matrix`.
    pub fn transpose(&self) -> Self {
        let (rows, cols) = self.size();
        let mut data = Vec::<T>::with_capacity(rows * cols);
        for j in 0..cols {
            for i in 0..rows {
                let el = self.get(i, j).clone();
                data.push(el);
            }
        }
        Self { data, rows: cols, cols: rows }
    }
}

impl<T> Matrix<T> where T: Clone + Default {

    /// Creates a new `Matrix` with the specified number of rows and columns.
    /// All values are initialized to their default values, i.e. 
    /// `T::default()`.
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![T::default(); rows * cols];
        Self { data, rows, cols }
    }
}

impl<T> Display for Matrix<T> where T: Clone + Display {

    /// Formats the `Matrix` to the specified `Formatter`.
    ///
    /// ## Examples
    /// The matrix `a` where:
    /// ```
    /// let a = matrix![
    ///     1, 2, 3;
    ///     4, 5, 6;
    ///     7, 8, 9
    /// ]
    /// println!("{}", a);
    /// ```
    /// Produces the following output:
    /// ```
    /// | 1  2  3 |
    /// | 4  5  6 |
    /// | 7  8  9 |
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (rows, cols) = self.size();

        // Determine maximum width of each column
        let mut col_widths = Vec::<usize>::with_capacity(cols);
        for j in 0..cols {
            let width = self.col(j)
                .map(|el| el.to_string().len())
                .fold(0usize, usize::max);

            col_widths.push(width + 1);
        }

        // Print each row
        for i in 0..rows {
            write!(f, "|")?;
            for j in 0..cols {
                let el = self.get(i, j);
                let s = el.to_string();
                let col_width = col_widths[j];
                write!(f, "{:>width$} ", s, width=col_width)?;
            }
            write!(f, "|\n")?;
        }

        Ok(())
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> where T: Clone + Default {

    /// Converts the specified two-dimensional `Vec` into an N by M `Matrix`,
    /// where M is equal to the length of the `Vec`, and M is equal to the
    /// maximum length of all the elements of the `Vec`, or `0` if there are no
    /// elements.
    fn from(vec: Vec<Vec<T>>) -> Self {
        let rows = vec.len();
        let cols = vec.iter()
            .map(|row| row.len())
            .fold(0usize, usize::max);

        let mut data = Vec::<T>::with_capacity(rows * cols);
        for i in 0..rows {
            let len = vec[i].len();

            // Push values from the row
            for j in 0..len {
                data.push(vec[i][j].clone());
            }

            // Push trailing default values
            for _ in 0..cols - len {
                data.push(T::default());
            }
        }

        Self { data, rows, cols }
    }
}

/// Represents a slice of a row of a `Matrix`.
pub struct Row<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    row: usize,
    col_index: usize
}

impl<'a, T: 'a> Index<usize> for Row<'a, T> {
    type Output = T;

    /// Produces a reference to the `j`th column of the `Row`.
    fn index(&self, j: usize) -> &T {
        self.matrix.get(self.row, j)
    }
}

impl<'a, T: 'a> Iterator for Row<'a, T> {
    type Item = &'a T;

    /// Produces a reference to the next value in the `Row`, or `None` if the
    /// `Iterator` has reached its end.
    fn next(&mut self) -> Option<&'a T> {
        if self.col_index < self.matrix.cols() {
            let val = self.matrix.get(self.row, self.col_index);
            self.col_index += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<'a, T: 'a + Clone> Into<Matrix<T>> for Row<'a, T> {

    /// Produces a `Matrix` containing copies of the values referenced in the
    /// `Row`.
    fn into(self) -> Matrix<T> {
        let data: Vec<_> = self.cloned().collect();
        let (rows, cols) = (1, data.len());
        Matrix { data, rows, cols }
    }
}

/// Represents a slice of a column of a `Matrix`.
pub struct Col<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    row_index: usize,
    col: usize
}

impl<'a, T: 'a> Index<usize> for Col<'a, T> {
    type Output = T;

    /// Produces a reference to the `i`th row in the `Col`.
    fn index(&self, i: usize) -> &T {
        self.matrix.get(i, self.col)
    }
}

impl<'a, T: 'a> Iterator for Col<'a, T> {
    type Item = &'a T;

    /// Produces a reference to the next value in the `Col`, or `None` if the
    /// `Iterator` has reached its end.
    fn next(&mut self) -> Option<&'a T> {
        if self.row_index < self.matrix.rows() {
            let val = self.matrix.get(self.row_index, self.col);
            self.row_index += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<'a, T: 'a + Clone> Into<Matrix<T>> for Col<'a, T> {

    /// Produces a `Matrix` containing copies of the values referenced in the
    /// `Col`.
    fn into(self) -> Matrix<T> {
        let data: Vec<_> = self.cloned().collect();
        let (rows, cols) = (data.len(), 1);
        Matrix { data, rows, cols }
    }
}

/// Represents a diagonal of a square `Matrix`.
pub struct Diag<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    index: usize
}

impl<'a, T: 'a> Iterator for Diag<'a, T> {
    type Item = &'a T;

    /// Produces a reference to the next value in the `Diag`, or `None` if the
    /// `Iterator` has reached its end.
    fn next(&mut self) -> Option<&'a T> {
        if self.index < self.matrix.rows() {
            let val = self.matrix.get(self.index, self.index);
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<'a, T: 'a + Clone + Default> Into<Matrix<T>> for Diag<'a, T> {

    /// Produces a square `Matrix` where each value along the diagonal is a
    /// clone of the value referenced to by the `Diag`.
    fn into(self) -> Matrix<T> {
        let vec: Vec<_> = self.cloned().collect();
        let len = vec.len();
        let mut matrix = Matrix::<T>::new(len, len);

        vec.into_iter().enumerate()
            .for_each(|(i, el)| matrix.set(i, i, el));

        matrix
    }
}

pub struct MatrixIterator<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    index: usize
}

impl<'a, T: 'a> Iterator for MatrixIterator<'a, T> {
    type Item = &'a T;

    /// Produces a reference to the next element in the `Matrix`. The `Matrix`
    /// is iterated over column-by-column row-by-row.
    ///
    /// The values in the `Matrix` below represent their order in the
    /// iteration:
    /// ```
    /// | 1  2  3 |
    /// | 4  5  6 |
    /// ```
    fn next(&mut self) -> Option<&'a T> {
        let result = self.matrix.data.get(self.index);
        match result {
            Some(_) => {
                self.index += 1;
                result
            },
            None => None
        }
    }
}