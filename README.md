# Matrix

[![Commit Count](https://img.shields.io/github/commit-count/xXDevSShXx/matrix)](https://github.com/xXDevSShXx/matrix)

A Rust library for working with matrices and linear algebra operations.

## Installation

To use the Matrix library, add the following to your Cargo.toml file:

```toml
[dependencies]
matrix = {git = "https://github.com/xXDevSShXx/matrix.git"}
```

Then, import the library in your Rust code:

```rust
use matrix::Matrix;
```

## Building

To build the Matrix library, run the following command in the project directory:

```
cargo build --release
```

## Usage

The Matrix library provides the following main features:

- Creating and manipulating matrices
- Performing matrix operations such as addition, subtraction, multiplication, and transposition
- Calculating the determinant and inverse of a matrix

Here's an example of how to use the library:

```rust
let collection = vec![
    vec![1.0, 3.0, 5.0],
    vec![2.0, 4.0, 6.0],
    vec![3.0, 7.0, 11.0],
];
let matrix: Matrix = Matrix::from(base_collection);

let determinant = matrix.determinant();
println!("Determinant: {}", determinant);
```

## Contributing

Contributions to the Matrix library are welcome! If you find any issues or have ideas for new features, please open an issue on the [GitHub repository](https://github.com/xXDevSShXx/matrix). Pull requests are also encouraged.

## License

The Matrix library is licensed under the [MIT License](LICENSE).
