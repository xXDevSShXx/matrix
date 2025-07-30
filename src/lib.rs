#![allow(unused)]

mod tests;

use std::{iter, ops, vec};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dimensions {
    pub rows: usize,
    pub columns: usize,
}

impl Dimensions {
    pub fn new(rows: usize, columns: usize) -> Self {
        assert!(rows > 0, "Rows should be more than 0.");
        assert!(columns > 0, "Columns should be more than 0.");

        Self { rows, columns }
    }

    pub fn square(size: usize) -> Self {
        Self::new(size, size)
    }
}

impl From<(usize, usize)> for Dimensions {
    fn from((rows, columns): (usize, usize)) -> Self {
        Self::new(rows, columns)
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    buffer: Vec<f64>,
    pub dimensions: Dimensions,
}

impl Matrix {
    // Constructors

    pub fn with_value(dimensions: Dimensions, value: f64) -> Self {
        Self {
            buffer: Vec::from_iter(iter::repeat_n(value, dimensions.columns * dimensions.rows)),
            dimensions,
        }
    }

    pub fn zero(dimensions: Dimensions) -> Self {
        Self::with_value(dimensions, 0.0)
    }

    pub fn scalar(main_diagonal: Vec<f64>) -> Self {
        let size = main_diagonal.len();
        let buffer: Vec<f64> = (0..(size * size))
            .map(|i| {
                if i % (size + 1) == 0 {
                    main_diagonal[i / (size + 1)]
                } else {
                    0.0
                }
            })
            .collect();

        Self {
            buffer,
            dimensions: Dimensions::square(size),
        }
    }

    pub fn identity(size: usize) -> Self {
        let buffer: Vec<f64> = (0..(size * size))
            .map(|i| f64::from(i % (size + 1) == 0))
            .collect();

        Self {
            buffer,
            dimensions: Dimensions::square(size),
        }
    }

    // Element access

    pub fn rows(&self) -> Vec<Vec<f64>> {
        self.buffer.chunks_exact(self.dimensions.columns).map(|row| row.to_owned()).collect_vec()
    }

    pub fn columns(&self) -> Vec<Vec<f64>> {
        (0..self.dimensions.columns)
            .map(|col_idx| {
                self.buffer
                    .iter()
                    .skip(col_idx)
                    .step_by(self.dimensions.columns)
                    .cloned()
                    .collect()
            })
            .collect()
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&f64> {
        let index = (i * self.dimensions.columns) + j;

        self.buffer.get(index)
    }

    pub fn row(&self, n: usize) -> Option<Vec<f64>> {
        let start_index = n * self.dimensions.columns;
        self.buffer
            .get(start_index..(start_index + self.dimensions.columns))
            .map(|row| row.to_owned())
    }

    pub fn column(&self, n: usize) -> Option<Vec<f64>> {
        if n >= self.dimensions.columns {
            return None;
        }
        let offset = n;

        let column = (0..self.dimensions.rows)
            .map(|row_i| self.buffer[(row_i * self.dimensions.columns) + offset])
            .collect();

        Some(column)
    }

    pub fn main_diagonal(&self) -> Vec<f64> {
        assert!(
            self.is_square(),
            "Main diagonal is a property of square matrices."
        );

        let divisor = self.dimensions.rows + 1;

        (0..self.dimensions.rows)
            .map(|n| self.buffer[n * divisor])
            .collect()
    }

    // Properties

    pub fn is_same_size(&self, other: &Self) -> bool {
        self.dimensions == other.dimensions
    }

    pub fn is_column(&self) -> bool {
        self.dimensions.columns == 1
    }

    pub fn is_row(&self) -> bool {
        self.dimensions.rows == 1
    }

    pub fn is_square(&self) -> bool {
        self.dimensions.rows == self.dimensions.columns
    }

    pub fn is_scalar(&self) -> bool {
        let (a, b) = (self.is_diagonal(), self.main_diagonal().iter().all_equal());
        a && b
    }

    pub fn is_upper_triangular(&self) -> bool {
        assert!(
            self.is_square(),
            "Only square matrices can be triangular."
        );

        let size = self.dimensions.rows;

        self.rows().iter().rev().enumerate().flat_map(|(i,row)| row.iter().skip(size - i)).all(|item| item == &0.0)
    }

    pub fn is_lower_triangular(&self) -> bool {
        assert!(
            self.is_square(),
            "Only square matrices can be triangular."
        );

        self.rows().iter().enumerate().flat_map(|(i,row)| row.iter().skip(i + 1)).all(|item| item == &0.0)
    }

    pub fn is_diagonal(&self) -> bool {
        assert!(
            self.is_square(),
            "Only square matrices can be diagonal."
        );


        let divisor = self.dimensions.columns + 1;
        self.buffer
            .iter()
            .enumerate()
            .all(|(index, item)| index % divisor == 0 || item == &0.0)
    }

    pub fn is_identity(&self) -> bool {
        self == &Self::identity(self.dimensions.rows)
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(collection: Vec<Vec<f64>>) -> Self {
        assert!(
            collection.iter().map(|row| row.len()).all_equal(),
            "Row sizes should be equal."
        );

        let dimensions = Dimensions::new(collection.len(), collection[0].len());

        Self {
            buffer: collection.concat(),
            dimensions,
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

fn dot_product(first: Vec<f64>, second: Vec<f64>) -> f64 {
    first
        .iter()
        .zip(second.iter())
        .map(|(first_item, second_item)| first_item * second_item)
        .sum()
}

// TODO: Heavy Refactoring Needed
impl ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert!(
            self.dimensions.columns == other.dimensions.rows,
            "To multiply matrices, the number of columns of the left matrix should be the same as the number of rows of the right matrix."
        );

        let (self_rows, mutual_dimention, other_columns) = (
            self.dimensions.rows,
            self.dimensions.columns,
            other.dimensions.columns,
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

        Self {
            buffer: result_collection,
            dimensions: Dimensions::new(self_rows, other_columns),
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.is_same_size(other)
    }
}
