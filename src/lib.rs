#![allow(unused)]

mod tests;

use std::{iter, ops, vec};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dimentions{
    pub rows: usize,
    pub columns: usize
}

impl Dimentions{
    pub fn new(rows: usize, columns: usize) -> Self{
        assert!(rows > 0, "Rows should be more than 0.");
        assert!(columns > 0, "Columns should be more than 0.");

        Self { rows, columns }
    }

    pub fn square(size: usize) -> Self{
        Self::new(size, size)
    }
}

impl From<(usize,usize)> for Dimentions {
    fn from((rows, columns): (usize,usize)) -> Self {
        Self::new(rows, columns )
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    matrix: Vec<Vec<f64>>,
    pub dimentions: Dimentions
}

impl Matrix {
    // Constructors

    pub fn with_value(dimentions: Dimentions, value: f64) -> Self {
       Self {
            matrix: Vec::from_iter(iter::repeat_n(iter::repeat_n(value, dimentions.columns).collect(), dimentions.rows)),
            dimentions
        }
    }

    pub fn zero(dimentions: Dimentions) -> Self {
        Self::with_value(dimentions, 0.0)
    }

    pub fn scalar(main_diagonal: Vec<f64>) -> Self{
        let size = main_diagonal.len();
        let collection: Vec<Vec<f64>> = (0..size).map(|i| {
            (0..size).map(|j| if i == j{
                main_diagonal.get(i).unwrap().clone()
            }else{ 0.0 }).collect()
        }).collect();

        Self::from(collection)
    }

    pub fn identity(size: usize) -> Self{
        let collection: Vec<Vec<f64>> = (0..size).map(|i| {
            (0..size).map(|j| f64::from(i == j)).collect()
        }).collect();

        Self::from(collection)
    }

    // Element access

    pub fn get(&self, i: usize, j: usize) -> Option<&f64> {
        assert!(i > 0 && j > 0, "i and j are natural numbers(i, j > 0).");
        self.matrix.get(i-1).and_then(|row| row.get(j-1))
    }

    pub fn main_diagonal(&self) -> Vec<f64> {
        assert!(self.is_square(), "Main diagonal is a property of square matrices.");
        
        self.matrix.iter().enumerate().map(|(i,row)| 
            *row.get(i).unwrap()
        ).collect()
    }

    // Properties

    pub fn is_same_size(&self, other: &Self) -> bool {
        self.dimentions == other.dimentions
    }

    pub fn is_column(&self) -> bool {
        self.dimentions.columns == 1
    }

    pub fn is_row(&self) -> bool {
        self.dimentions.rows == 1
    }


    pub fn is_square(&self) -> bool {
        self.dimentions.rows == self.dimentions.columns
    }

    pub fn is_scalar(&self) -> bool {
        self.is_diagonal() && self.main_diagonal().iter().tuple_windows().all(|(a,b)| a == b)
    }

    pub fn is_upper_triangular(&self) -> bool {
        assert!(self.is_square(), "Triangular matrices should also be square matrices.");

        self.matrix.iter().enumerate()
            .all(| (i,row)| 
                    row.iter().enumerate()
                    .filter(|(j,_)| &i > j)
                    .all(|(_, item)| item == &0f64))
    }

    pub fn is_lower_triangular(&self) -> bool {
        assert!(self.is_square(), "Triangular matrices should also be square matrices.");


        self.matrix.iter().enumerate()
            .all(| (i,row)| 
                    row.iter().enumerate()
                    .filter(|(j,_)| &i < j)
                    .all(|(_, item)| item == &0f64))
    }

    pub fn is_diagonal(&self) -> bool {
        assert!(self.is_square(), "Diagonal matrices should also be square matrices.");

        self.matrix.iter().enumerate()
            .all(| (i,row)| 
                    row.iter().enumerate()
                    .filter(|(j,_)| &i != j)
                    .all(|(_, item)| item == &0f64))
    }

    pub fn is_identity(&self) -> bool{
        self == &Self::identity(self.dimentions.rows)
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(collection: Vec<Vec<f64>>) -> Self {
        assert!(!collection.is_empty() && collection.iter().all(|row| !row.is_empty()));

        let dimentions = Dimentions::new(collection.len(), collection.first().map_or(0, |row| row.len()));

        Self{
            matrix: collection,
            dimentions 
        }
    }
}

// TODO: Heavy Refactoring Needed
impl ops::Add for Matrix{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert!(self.is_same_size(&other),"To add matrices they should be of the the same dimentions.");

        let self_collection = (self.matrix).iter();
        let mut other_collection = (other.matrix).iter();

        let result_collection: Vec<Vec<f64>> = self_collection.map(|self_row| {
            let mut other_row = other_collection.next().unwrap().iter();
            
            self_row.iter().map(|self_element| {
                let other_element = other_row.next().unwrap();

                self_element + other_element

            }).collect()
        }).collect();

        Self{matrix: result_collection,dimentions: self.dimentions}
    }
}

// TODO: Heavy Refactoring Needed
impl ops::Mul for Matrix{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert!(self.dimentions.columns == other.dimentions.rows,"To multiply matrices, the number of columns of the left matrix should be the same as the number of rows of the right matrix.");

        let self_collection = (self.matrix).iter();
        let mut other_collection = (other.matrix).iter();

        let result_collection = self_collection.map(|self_row| 
            (1..=other.dimentions.columns).map(|j|
                self_row.iter().enumerate().map(|(i,self_element)| self_element * other.get(i+1, j).unwrap()).sum()
            ).collect()
        ).collect();

        Self{matrix: result_collection,dimentions: Dimentions::new(self.dimentions.rows, other.dimentions.columns)}
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix && self.is_same_size(other)
    }
}