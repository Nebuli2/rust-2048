/// Produces a 2-dimensional `Vec`. Elements in each inner `Vec` are delimited
/// by commas, and inner `Vec`s are delimited by semicolons.
///
/// ## Example
/// ```
/// let a = vec2d![
///     1, 2, 3;
///     4, 5, 6
/// ];
/// ```
/// Expands to:
/// ```
/// let a = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6]
/// ];
/// ```
///
macro_rules! vec2d {
    [$($($el:expr),*);*] => (vec![$(vec![$($el),*]),*])
}

/// Creates a `Matrix`. Rows are delimited by semicolons, and within each row,
/// columns are delimited by commas. If the rows are not equal in length, the
/// `Matrix` will have a width equal to the width of the longest row. All
/// trailing values in other rows will be assigned their default values.
///
/// ## Example
/// The matrix:
/// ```
/// | 1  2  3 |
/// | 4  5  6 |
/// | 7  8  9 |
/// ```
/// Can be defined as:
/// ```
/// let a = matrix![
///     1, 2, 3;
///     4, 5, 6;
///     7, 8, 9
/// ];
/// ```
///
macro_rules! matrix {
    [$($($el:expr),*);*] => (::matrix::Matrix::from(
        vec2d![$($($el),*);*]
    ))
}