#[cfg(test)]
use crate::*;

#[test]
fn test_row_works() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection.clone());

    for (i, row) in base_collection.iter().enumerate() {
        assert_eq!(matrix.row(i).unwrap(), *row);
    }
}

#[test]
fn test_column_works() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection.clone());

    for i in 0..base_collection.len() {
        assert_eq!(
            matrix.column(i).unwrap(),
            base_collection.iter().map(|row| row[i]).collect_vec()
        );
    }
}

#[test]
fn test_addition_set_value_custom_values() {
    let matrix1: Matrix = Matrix::with_value(Dimensions::square(3), 5.0);

    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix2: Matrix = Matrix::from(base_collection);

    let expected_collection = vec![
        vec![6.0, 8.0, 10.0],
        vec![7.0, 9.0, 11.0],
        vec![8.0, 12.0, 16.0],
    ];
    let expected_result = Matrix::from(expected_collection);

    assert_eq!(matrix1 + matrix2, expected_result)
}

#[test]
fn test_addition_identity_set_value() {
    let matrix1: Matrix = Matrix::with_value(Dimensions::square(3), 5.0);

    let matrix2: Matrix = Matrix::identity(3);

    let expected_collection = vec![
        vec![6.0, 5.0, 5.0],
        vec![5.0, 6.0, 5.0],
        vec![5.0, 5.0, 6.0],
    ];
    let expected_result = Matrix::from(expected_collection);

    assert_eq!(matrix1 + matrix2, expected_result)
}

#[test]
fn test_multiplication_controlled_matrices() {
    let matrix1_collection = vec![vec![2.0, 1.0], vec![0.0, 3.0], vec![-1.0, 2.0]];
    let matrix1 = Matrix::from(matrix1_collection);

    let matrix2_collection = vec![vec![-1.0, 0.0, 1.0], vec![2.0, 3.0, -1.0]];
    let matrix2 = Matrix::from(matrix2_collection);

    let expected_collection = vec![
        vec![0.0, 3.0, 1.0],
        vec![6.0, 9.0, -3.0],
        vec![5.0, 6.0, -3.0],
    ];
    let expected_result = Matrix::from(expected_collection);

    assert_eq!(matrix1 * matrix2, expected_result)
}

#[test]
fn test_properties_set_value_is_column() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(!matrix.is_column());
}

#[test]
fn test_properties_set_value_is_row() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(!matrix.is_row());
}

#[test]
fn test_properties_set_value_is_diagonal() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(!matrix.is_diagonal());
}

#[test]
fn test_properties_set_value_is_identity() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(!matrix.is_identity());
}

#[test]
fn test_properties_set_value_is_x_triangular() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(!matrix.is_lower_triangular());
    assert!(!matrix.is_upper_triangular());
}

#[test]
fn test_properties_set_value_is_scalar() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(!matrix.is_scalar());
}

#[test]
fn test_properties_set_value_is_square() {
    let base_collection = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 6.0],
        vec![3.0, 7.0, 11.0],
    ];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(matrix.is_square());
}

#[test]
fn test_properties_identity_is_column() {
    let matrix: Matrix = Matrix::identity(5);

    assert!(!matrix.is_column());
}

#[test]
fn test_properties_identity_is_row() {
    let matrix: Matrix = Matrix::identity(5);

    assert!(!matrix.is_row());
}

#[test]
fn test_properties_identity_is_diagonal() {
    let matrix: Matrix = Matrix::identity(5);

    assert!(matrix.is_diagonal());
}

#[test]
fn test_properties_identity_is_identity() {
    let matrix: Matrix = Matrix::identity(5);

    assert!(matrix.is_identity());
}

#[test]
fn test_properties_identity_is_x_triangular() {
    let matrix: Matrix = Matrix::identity(5);

    assert!(matrix.is_lower_triangular());
    assert!(matrix.is_upper_triangular());
}

#[test]
fn test_properties_identity_is_scalar() {
    let matrix: Matrix = Matrix::identity(5);

    assert!(matrix.is_scalar());
}

#[test]
fn test_properties_identity_is_square() {
    let matrix: Matrix = Matrix::identity(5);

    assert!(matrix.is_square());
}

#[test]
fn test_properties_column_is_column() {
    let base_collection = vec![vec![1.0], vec![2.0], vec![3.0]];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(matrix.is_column());
}

#[test]
fn test_properties_column_is_square() {
    let base_collection = vec![vec![1.0], vec![2.0], vec![3.0]];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(!matrix.is_square());
}

#[test]
fn test_properties_row_is_row() {
    let base_collection = vec![vec![1.0, 3.0, 5.0]];
    let matrix: Matrix = Matrix::from(base_collection);

    assert!(matrix.is_row());
}
