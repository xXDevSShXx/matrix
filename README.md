# Matrix

A Rust library for working with matrices and linear algebra operations.

## Installation

To use the Matrix library, add the following to your Cargo.toml file:

```toml
[dependencies]
matrix = "1.0.0"
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
- Solving systems of linear equations

Here's an example of how to use the library:

```rust
let mut m = Matrix::new(3, 3);
m[(0, 0)] = 1;
m[(0, 1)] = 2;
m[(0, 2)] = 3;
m[(1, 0)] = 4;
m[(1, 1)] = 5;
m[(1, 2)] = 6;
m[(2, 0)] = 7;
m[(2, 1)] = 8;
m[(2, 2)] = 9;

let determinant = m.determinant();
println!("Determinant: {}", determinant);
```

## Contributing

Contributions to the Matrix library are welcome! If you find any issues or have ideas for new features, please open an issue on the [GitHub repository](https://github.com/xXDevSShXx/matrix). Pull requests are also encouraged.

## License

The Matrix library is licensed under the [MIT License](LICENSE).
