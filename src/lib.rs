#![allow(unused, renamed_and_removed_lints)]

mod tests;

use std::{iter, ops, vec};

use itertools::{Itertools, Product};

/// Represents the dimensions of a matrix, either a square matrix or a rectangular matrix.
///
/// # Variants
/// - `Square(usize)`: Square matrix with the given side length.
/// - `Rectangle { rows: usize, columns: usize }`: Rectangular matrix with specified rows and columns.
#[derive(Debug, Clone, Copy)]
pub enum Dimensions {
    Square(usize),
    Rectangle { rows: usize, columns: usize },
}

/// A matrix of `f64` values with defined dimensions and internal buffer storage.
///
/// Provides construction, element access, manipulation, and common matrix properties and operations.
impl Dimensions {
    /// Returns the number of rows in the dimensions.
    pub fn rows(&self) -> usize {
        *(match self {
            Dimensions::Rectangle { rows, .. } => rows,
            Dimensions::Square(len) => len,
        })
    }

    /// Returns the number of columns in the dimensions.
    pub fn columns(&self) -> usize {
        *(match self {
            Dimensions::Rectangle { columns, .. } => columns,
            Dimensions::Square(len) => len,
        })
    }

    /// Returns the total number of elements (rows * columns).
    pub fn count(&self) -> usize {
        self.rows() * self.columns()
    }

    /// Returns a new `Dimensions` which is the transpose of the current dimensions.
    ///
    /// The rows and columns are swapped for rectangular matrices.
    pub fn transposed(&self) -> Dimensions {
        match self {
            Self::Rectangle { rows, columns } => Self::Rectangle {
                rows: *columns,
                columns: *rows,
            },
            Self::Square(_) => *self,
        }
    }
}

impl From<(usize, usize)> for Dimensions {
    fn from((rows, columns): (usize, usize)) -> Self {
        if rows == columns {
            return Self::Square(rows);
        }
        Self::Rectangle { rows, columns }
    }
}

impl From<usize> for Dimensions {
    fn from(len: usize) -> Self {
        Self::Square(len)
    }
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Square(self_size), Self::Square(other_size)) => self_size == other_size,
            (
                Self::Rectangle {
                    rows: self_rows,
                    columns: self_columns,
                },
                Self::Rectangle {
                    rows: other_rows,
                    columns: other_columns,
                },
            ) => self_rows == other_rows && self_columns == other_columns,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    buffer: Vec<f64>,
    /// The dimensions of the matrix.
    pub dimensions: Dimensions,
}

#[derive(Debug)]
pub enum ErrorKind {
    DimensionsIncorrct(String),
    DividedByZero,
}

impl Matrix {
    // Constructors

    /// Creates a new matrix filled with a constant `value` for the specified `dimensions`.
    ///
    /// # Arguments
    /// * `dimensions` - The size specifications of the matrix.
    /// * `value` - The constant value to fill the matrix with.
    ///
    /// # Example
    /// ```
    /// use matrix::{Dimensions, Matrix};
    ///
    ///
    /// let m = Matrix::constant(Dimensions::Square(3), 5.0);
    /// ```
    pub fn constant(dimensions: Dimensions, value: f64) -> Self {
        Self {
            buffer: iter::repeat(value)
                .take(dimensions.count())
                .collect::<Vec<f64>>(),
            dimensions,
        }
    }

    /// Creates a zero matrix with the given dimensions.
    ///
    /// Equivalent to `Matrix::constant(dimensions, 0.0)`.
    pub fn zero(dimensions: Dimensions) -> Self {
        Self::constant(dimensions, 0.0)
    }

    /// Creates a diagonal matrix with specified values on the main diagonal.
    ///
    /// The size of the matrix will be equal to the length of `main_diagonal`.
    ///
    /// # Example
    /// ```
    /// use matrix::{Dimensions, Matrix};
    ///
    /// let diag = Matrix::diagonal(vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn diagonal(main_diagonal: Vec<f64>) -> Self {
        let size = main_diagonal.len();
        let mut result: Self = Self::zero(size.into());

        for (index, item) in main_diagonal.iter().enumerate() {
            result.set(index, index, *item);
        }

        result
    }

    /// Creates an scalar matrix of the given size and value.
    ///
    /// An scalar matrix is a diagonal matrix with constant values on the main diagonal.
    ///
    /// # Example
    /// ```
    /// use matrix::{Dimensions, Matrix};
    ///
    /// let scalar = Matrix::scalar(0.5, 3);
    /// ```
    pub fn scalar(value: f64, size: usize) -> Self {
        Self::diagonal(iter::repeat(value).take(size).collect::<Vec<f64>>())
    }

    /// Creates an identity matrix of the given size and value.
    ///
    /// An identity matrix is a scalar matrix with ones on the main diagonal.
    ///
    /// # Example
    /// ```
    /// use matrix::{Dimensions, Matrix};
    ///
    /// let i3 = Matrix::identity(3);
    /// ```
    pub fn identity(size: usize) -> Self {
        Self::diagonal(iter::repeat(1.0).take(size).collect::<Vec<f64>>())
    }

    // Element access

    /// Returns the rows of the matrix as a vector of vectors.
    ///
    /// Each inner vector represents one row.
    pub fn rows(&self) -> Vec<Vec<f64>> {
        self.buffer
            .chunks_exact(self.dimensions.columns())
            .map(|row| row.to_owned())
            .collect::<Vec<_>>()
    }

    // TODO: needs further refactoring in the future

    /// Returns the columns of the matrix as a vector of vectors.
    ///
    /// Each inner vector represents one column.
    pub fn columns(&self) -> Vec<Vec<f64>> {
        let columns = self.dimensions.columns();
        (0..columns)
            .map(|col_idx| {
                self.buffer
                    .iter()
                    .skip(col_idx)
                    .step_by(columns)
                    .cloned()
                    .collect()
            })
            .collect()
    }

    /// Returns an option containing a reference to the element at row `i` and column `j`.
    ///
    /// Returns `None` if indices are out of bounds.
    pub fn get(&self, i: usize, j: usize) -> Option<&f64> {
        if (i >= self.dimensions.rows() || j >= self.dimensions.columns()) {
            return None;
        }

        let index = (i * self.dimensions.columns()) + j;
        self.buffer.get(index)
    }

    /// Returns a vector containing all elements of the `n`th row.
    ///
    /// Returns `None` if `n` is out of range.
    pub fn row(&self, n: usize) -> Option<Vec<f64>> {
        if n >= self.dimensions.rows() {
            return None;
        }

        self.rows().get(n).map(|item| item.to_owned())
    }

    /// Returns a vector containing all elements of the `n`th column.
    ///
    /// Returns `None` if `n` is out of range.
    pub fn column(&self, n: usize) -> Option<Vec<f64>> {
        if n >= self.dimensions.columns() {
            return None;
        }

        self.columns().get(n).map(|item| item.to_owned())
    }

    /// Returns a vector of references to the elements on the main diagonal of a square matrix.
    ///
    /// Returns `None` if `the matrix is not square.
    pub fn main_diagonal(&self) -> Option<Vec<&f64>> {
        if !self.is_square() {
            return None;
        }

        Some(
            (0..self.dimensions.rows())
                .map(|index| self.get(index, index).unwrap())
                .collect_vec(),
        )
    }

    /// Returns a vector of references to the elements on the secondary diagonal of a square matrix.
    ///
    /// Returns `None` if `the matrix is not square.
    pub fn secondary_diagonal(&self) -> Option<Vec<&f64>> {
        if !self.is_square() {
            return None;
        }

        let last_index = self.dimensions.rows() - 1;
        Some(
            (0..=last_index)
                .map(|index| self.get(index, last_index - index).unwrap())
                .collect_vec(),
        )
    }

    /// Returns the determinant of the matrix, calculated using an unoptimized algorithm.
    ///
    /// Returns `None` if `the matrix is not square.
    pub fn determinant_unoptimized(&self) -> Option<f64> {
        // checking for the matrix being square is done here and so any other checks are unnecessary.
        if !self.is_square() {
            return None;
        }

        Some(match self.dimensions.rows() {
            0 => 0.0,
            1 => *self.get(0, 0).unwrap(),
            2 => {
                self.main_diagonal()
                    .unwrap()
                    .iter()
                    .fold(1f64, |value, &item| value * item)
                    - self
                        .secondary_diagonal()
                        .unwrap()
                        .iter()
                        .fold(1f64, |value, &item| value * item)
            }
            dimensions => {
                let r1: Vec<f64> = self.row(0).unwrap();
                r1.iter()
                    .enumerate()
                    .map(|(index, value)| {
                        let remaining_matrix = Matrix::from_buffer(
                            self.buffer
                                .iter()
                                .enumerate()
                                .skip(dimensions)
                                .filter(|(i, _)| *i % dimensions != index)
                                .map(|(_, item)| item.to_owned())
                                .collect(),
                            Dimensions::Square(dimensions - 1),
                        )
                        .unwrap();

                        value
                            * remaining_matrix.determinant_unoptimized().unwrap()
                            * if index % 2 == 0 { 1.0 } else { -1.0 }
                    })
                    .sum()
            }
        })
    }

    // Manipulation

    /// Transposes the matrix in place, swapping rows and columns.
    pub fn transpose(&mut self) {
        // Calculated Dimensions always match the element count.
        *self = Matrix::from_buffer(self.columns().concat(), self.dimensions.transposed()).unwrap()
    }

    /// Returns a new matrix which is the transpose of the current matrix.
    pub fn transposed(&self) -> Self {
        let mut result = self.clone();
        result.transpose();

        result
    }

    /// Sets the value at row `i` and column `j` to `value`.
    ///
    /// Returns `true` if the value was updated, or `false` if indices were out of bounds.
    pub fn set(&mut self, i: usize, j: usize, value: f64) -> bool {
        if (i >= self.dimensions.rows() || j >= self.dimensions.columns()) {
            return false;
        }

        let index = (i * self.dimensions.columns()) + j;
        *(self.buffer.get_mut(index).unwrap()) = value;

        true
    }

    // Properties

    /// Returns `true` if this matrix has exactly the same dimensions as another.
    pub fn is_same_size(&self, other: &Self) -> bool {
        self.dimensions == other.dimensions
    }

    /// Returns `true` if the matrix has only one column.
    pub fn is_column(&self) -> bool {
        self.dimensions.columns() == 1
    }

    /// Returns `true` if the matrix has only one row.
    pub fn is_row(&self) -> bool {
        self.dimensions.rows() == 1
    }

    /// Returns `true` if the matrix is square.
    pub fn is_square(&self) -> bool {
        match self.dimensions {
            Dimensions::Square(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the matrix is a scalar multiple of the identity matrix.
    pub fn is_scalar(&self) -> bool {
        // checking for the matrix being square is done here,
        if !self.is_diagonal() {
            return false;
        }

        // so the .main_diagonal() function will always return Some.
        match self.main_diagonal().unwrap().iter().all_equal_value() {
            Ok(&value) => *value == 1.0,
            _ => false,
        }
    }

    /// Returns `true` if the matrix is upper triangular.
    pub fn is_upper_triangular(&self) -> bool {
        if !self.is_square() {
            return false;
        }

        let size = self.dimensions.rows();

        self.rows()
            .iter()
            .rev()
            .enumerate()
            .flat_map(|(i, row)| row.iter().skip(size - i))
            .all(|item| item == &0.0)
    }

    /// Returns `true` if the matrix is lower triangular.
    pub fn is_lower_triangular(&self) -> bool {
        if !self.is_square() {
            return false;
        }

        self.rows()
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().skip(i + 1))
            .all(|item| item == &0.0)
    }

    /// Returns `true` if the matrix is diagonal.
    pub fn is_diagonal(&self) -> bool {
        if !self.is_square() {
            return false;
        }

        let divisor = self.dimensions.columns() + 1;
        self.buffer
            .iter()
            .enumerate()
            .all(|(index, item)| index % divisor == 0 || item == &0.0)
    }

    /// Returns `true` if the matrix is an identity matrix.
    pub fn is_identity(&self) -> bool {
        self == &Self::identity(self.dimensions.rows())
    }
}

impl TryFrom<Vec<Vec<f64>>> for Matrix {
    type Error = ErrorKind;
    fn try_from(collection: Vec<Vec<f64>>) -> Result<Self, Self::Error> {
        if !collection.iter().map(|row| row.len()).all_equal() {
            return Err(ErrorKind::DimensionsIncorrct(
                "Row sizes should be equal.".to_string(),
            ));
        }

        let dimensions = Dimensions::from((collection.len(), collection[0].len()));

        Ok(Self {
            buffer: collection.concat(),
            dimensions,
        })
    }
}

impl Matrix {
    fn from_buffer(buffer: Vec<f64>, dimensions: Dimensions) -> Result<Self, ErrorKind> {
        if buffer.len() != dimensions.count() {
            return Err(ErrorKind::DimensionsIncorrct(
                "Dimensions don't match the input size.".to_string(),
            ));
        }

        Ok(Self { buffer, dimensions })
    }
}

impl ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            buffer: self.buffer.iter().map(|item| item * rhs).collect_vec(),
            dimensions: self.dimensions,
        }
    }
}

impl ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul for Matrix {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        // If the number of columns of the first matrix doesn't match the number of rows of the second matrix,
        // product of the two matrices is undifined.
        if self.dimensions.columns() != other.dimensions.rows() {
            return None;
        }

        let (self_rows, mutual_dimension, other_columns) = (
            self.dimensions.rows(),
            self.dimensions.columns(),
            other.dimensions.columns(),
        );

        let mut result_collection: Vec<f64> = Vec::new();

        let mut result_collection = Vec::with_capacity(self_rows * other_columns);

        for self_i in 0..self_rows {
            for other_j in 0..other_columns {
                result_collection.push(dot_product(
                    self.row(self_i).unwrap(),
                    other.column(other_j).unwrap(),
                ));
            }
        }

        Some(Self {
            buffer: result_collection,
            dimensions: Dimensions::from((self_rows, other_columns)),
        })
    }
}

/// Computes the dot product of two vectors.
///
/// # Arguments
/// * `first` - First vector of f64 values.
/// * `second` - Second vector of f64 values.
///
/// # Returns
/// Sum of element-wise products.
///
/// # Panics
/// Panics if the vectors are of different lengths.
fn dot_product(first: Vec<f64>, second: Vec<f64>) -> f64 {
    first
        .iter()
        .zip(second.iter())
        .map(|(first_item, second_item)| first_item * second_item)
        .sum()
}

impl ops::Div<f64> for Matrix {
    type Output = Matrix;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            buffer: self.buffer.iter().map(|item| item / rhs).collect_vec(),
            dimensions: self.dimensions,
        }
    }
}

impl ops::Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert!(
            self.is_same_size(&other),
            "To add matrices they should be of the the same dimensions."
        );

        let buffer = self
            .buffer
            .iter()
            .zip(other.buffer.iter())
            .map(|(self_item, other_item)| self_item + other_item)
            .collect();

        Self {
            buffer,
            dimensions: self.dimensions,
        }
    }
}

impl ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.is_same_size(other)
    }
}
