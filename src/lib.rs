#![allow(unused, manual_repeat_n, renamed_and_removed_lints)]

mod tests;

use std::{iter, ops, vec};

use itertools::{Itertools, Product};

#[derive(Debug, Clone, Copy)]
pub enum Dimensions {
    Square(usize),
    Rectangle{ rows: usize, columns: usize }
}

impl Dimensions {
    pub fn rows(&self) -> usize {
        *(match self {
            Dimensions::Rectangle{ rows, .. } => rows,
            Dimensions::Square(len) => len
        })
    }
    
    pub fn columns(&self) -> usize {
        *(match self {
            Dimensions::Rectangle{ columns, .. } => columns,
            Dimensions::Square(len) => len
        })
    }

    pub fn count(&self) -> usize {
        self.rows() * self.columns()
    }

    pub fn transposed(&self) -> Dimensions{
        match self {
            Self::Rectangle { rows, columns } => Self::Rectangle { rows: *columns, columns: *rows },
            Self::Square(_) => *self
        }
    }
}

impl From<(usize, usize)> for Dimensions {
    fn from((rows, columns): (usize, usize)) -> Self {
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
            (Self::Rectangle { rows: self_rows, columns: self_columns }, Self::Rectangle { rows: other_rows, columns: other_columns }) => self_rows == other_rows && self_columns == other_columns,
            (Self::Square(self_size), Self::Rectangle { rows: other_rows, columns: other_columns }) => other_rows == other_columns && self_size == other_rows,
            (Self::Rectangle { rows: self_rows, columns: self_columns }, Self::Square(other_size)) => self_rows == self_columns && other_size == self_rows,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    buffer: Vec<f64>,
    pub dimensions: Dimensions,
}

impl Matrix {
    // Constructors

    pub fn constant(dimensions: Dimensions, value: f64) -> Self {
        Self {
            buffer: iter::repeat(value).take(dimensions.count()).collect::<Vec<f64>>(),
            dimensions,
        }
    }

    pub fn zero(dimensions: Dimensions) -> Self {
        Self::constant(dimensions, 0.0)
    }

    pub fn scalar(main_diagonal: Vec<f64>) -> Self {
        let size = main_diagonal.len();
        let mut result: Self = Self::zero(size.into());

        for (index, item) in main_diagonal.iter().enumerate(){
            result.set(index, index, *item);
        }

        result
    }

    pub fn identity(size: usize) -> Self {
        Self::scalar(iter::repeat(1.0).take(size).collect::<Vec<f64>>())
    }

    // Element access

    pub fn rows(&self) -> Vec<Vec<f64>> {
        self.buffer
            .chunks_exact(self.dimensions.columns())
            .map(|row| row.to_owned())
            .collect::<Vec<_>>()
    }

    // TODO: needs further refactoring in the future
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

    pub fn get(&self, i: usize, j: usize) -> Option<&f64> {
        if (i >= self.dimensions.rows() || j >= self.dimensions.columns()){
            return None;
        }

        let index = (i * self.dimensions.columns()) + j;
        self.buffer.get(index)
    }

    pub fn row(&self, n: usize) -> Option<Vec<f64>> {
        if n >= self.dimensions.rows() {
            return None;
        }

        self.rows().get(n).map(|item| item.to_owned())
    }

    pub fn column(&self, n: usize) -> Option<Vec<f64>> {
        if n >= self.dimensions.columns() {
            return None;
        }

        self.columns().get(n).map(|item| item.to_owned())
    }

    pub fn main_diagonal(&self) -> Vec<&f64> {
        assert!(
            self.is_square(),
            "Main diagonal is a property of square matrices."
        );

        (0..self.dimensions.rows())
            .map(|index| self.get(index, index).unwrap())
            .collect_vec()
    }

        pub fn secondary_diagonal(&self) -> Vec<&f64> {
        assert!(
            self.is_square(),
            "Secondary diagonal is a property of square matrices."
        );

        let last_index = self.dimensions.rows() - 1;
        (0..=last_index)
            .map(|index| self.get(index, last_index - index).unwrap())
            .collect_vec()
    }
    
        pub fn determinant_unoptimized(&self) -> f64 {
            assert!(self.is_square(), "Determinant is only defined for square matrices.");    
            match self.dimensions.rows() {
                0 => 0.0,
                1 => *self.get(0, 0).unwrap(),
                2 => self.main_diagonal().iter().fold(1f64, |value ,&item| value * item) - self.secondary_diagonal().iter().fold(1f64, |value ,&item| value * item) ,
                dimensions => {
                    let r1: Vec<f64> = self.row(0).unwrap();
                    r1.iter().enumerate().map(|(index, value)| {
                            let remaining_matrix = Matrix::from_buffer(self.buffer.iter().enumerate().skip(dimensions).filter(|(i, _)| *i % dimensions != index).map(|(_, item)| item.to_owned()).collect(), Dimensions::Square(dimensions - 1));
                            value * remaining_matrix.determinant_unoptimized() * if index % 2 == 0 { 1.0 } else { -1.0 }
                    }).sum()
                }
            }
        }

    // Manipulation

    pub fn transpose(&mut self) {
        *self = Matrix::from_buffer(self.columns().concat(), self.dimensions.transposed())
    }

    pub fn transposed(&self) -> Self{
        let mut result = self.clone();
        result.transpose();

        result
    }

    pub fn set(&mut self, i: usize, j: usize, value: f64) -> bool {
        if (i >= self.dimensions.rows() || j >= self.dimensions.columns()){
            return false;
        }

        let index = (i * self.dimensions.columns()) + j;
        *(self.buffer.get_mut(index).unwrap()) = value;
        
        true
    }

    // Properties

    pub fn is_same_size(&self, other: &Self) -> bool {
        self.dimensions == other.dimensions
    }

    pub fn is_column(&self) -> bool {
        self.dimensions.columns() == 1
    }

    pub fn is_row(&self) -> bool {
        self.dimensions.rows() == 1
    }

    pub fn is_square(&self) -> bool {
        match self.dimensions {
            Dimensions::Square(_) => true,
            Dimensions::Rectangle { rows, columns } => rows == columns,
        }
    }

    pub fn is_scalar(&self) -> bool {
        if !self.is_diagonal() {
            return false;
        }
        
        match self.main_diagonal().iter().all_equal_value() {
            Ok(&value) => *value == 1.0,
            _ => false,
        }
    }

    pub fn is_upper_triangular(&self) -> bool {
        if !self.is_square(){
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

    pub fn is_lower_triangular(&self) -> bool {
        if !self.is_square(){
            return false;
        }

        self.rows()
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().skip(i + 1))
            .all(|item| item == &0.0)
    }

    pub fn is_diagonal(&self) -> bool {
        if !self.is_square(){
            return false;
        }

        let divisor = self.dimensions.columns() + 1;
        self.buffer
            .iter()
            .enumerate()
            .all(|(index, item)| index % divisor == 0 || item == &0.0)
    }

    pub fn is_identity(&self) -> bool {
        self == &Self::identity(self.dimensions.rows())
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(collection: Vec<Vec<f64>>) -> Self {
        assert!(
            collection.iter().map(|row| row.len()).all_equal(),
            "Row sizes should be equal."
        );

        let dimensions = Dimensions::Rectangle{ rows: collection.len(), columns: collection[0].len() };

        Self {
            buffer: collection.concat(),
            dimensions,
        }
    }
}

impl Matrix {
    fn from_buffer(buffer: Vec<f64>, dimensions: Dimensions) -> Self {
        assert_eq!(
            buffer.len(),
            dimensions.count(),
            "Dimensions don't match the input size."
        );

        Self {
            buffer,
            dimensions,
        }
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
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert!(
            self.dimensions.columns() == other.dimensions.rows(),
            "To multiply matrices, the number of columns of the left matrix should be the same as the number of rows of the right matrix."
        );

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

        Self {
            buffer: result_collection,
            dimensions: Dimensions::Rectangle{ rows: self_rows, columns: other_columns },
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
