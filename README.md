# Simple-Matrix: A simple generic matrix library in Rust

## Who, What & Why?
- *Who?*
	- I am a French student that is interested in programming (and in Rust for a couple months).
- *What?*
	- It is a simple matrix library in Rust without dependencies.
	- It has no intention to be the *best/fastest/most feature-complete*.
	- Though, if optimizations keep the API simple, they will be included.
- *Why?*
	- To be better in Rust and discover some of its numerous aspects.
	- To create a simple and reliable matrix library.

### Disclaimer
This crate is not mature enough for *professional use*, check alternatives like [cgmath](https://github.com/brendanzab/cgmath) or [nalgebra](https://github.com/sebcrozet/nalgebra) if you are in that case.

If you are still interested, feel free to continue!

## Usage 
Link it in your project's `Cargo.toml` file:
```toml
# Example Cargo.toml (replace values with your own)

[dependencies]
simple-matrix = "0.1"
```

Then, you can use it in your project:
### Rust 2015
```rust
// Specify the extern crate in your lib.rs or main.rs
extern crate simple_matrix;

// You can now use it
use simple_matrix::Matrix;

let mat: Matrix<i32> = Matrix::new();
```

### Rust 2018
```rust
// No need to specify an extern crate
// You can use it directly
use simple_matrix::Matrix;

let mat: Matrix<i32> = Matrix::new();
```

### Example: Basic matrix usage
```rust
// Create a matrix of default cells
let zero: Matrix<u32> = Matrix::new(3, 3);

// Create a 2x4 matrix from an iterator (fill it row by row)
let mat1: Matrix<u32> = Matrix::from_iter(2, 4, 0..);

// Clone a matrix
let mat2 = mat1.clone();


// Add by reference (do not consume them)
let mut add = &mat1 + &mat2;

// Subtract by value (consume them)
let mut sub = mat1 - mat2;

// OpAssign are also available
sub += &zero;
sub -= zero;


// Get cells
let val: &u32 = add.get(0, 3).unwrap();

// Set cells
add.set(0, 3, 0);

// Iterate through the matrix (row by row)
for val in add {
    print!("{} ", val);
}
```

### Example: Dot product
```rust
let mat: Matrix<f64> = Matrix::from_iter(2, 4, 0..);

// Construct the transposed matrix
let mat_t = mat.transpose();

// Construct the dot product
let dot = mat * mat_t;
```

### Features
- *Features are extensions of the library left to opt-in by the user.*
- *They can increase compilation time and library size.*

To include a feature, add it to your `Cargo.toml` file:
```toml
# Example Cargo.toml with added feature (replace values with your own)

[dependencies]
simple-matrix = { version = "0.1", features = ["impl_from"] }
```

Current available features are listed below with a little description:
#### impl_from
Implements the *From* Trait for basic numeric types.

```rust
let m1: Matrix<i8> = Matrix::new(3, 5);
let m2: Matrix<i64> = m1.into();
```

### Tests
- Run `cargo test` in the root of the project
- Documentation tests are disabled for now (rustdoc does not seem to work with edition 2018)

### Benchmarks
- Run `cargo bench` in the root of the project
- Benchmarks are handled by the [Criterion](https://github.com/japaric/criterion.rs) crate, check its documentation for more detailled usage.

Thoses benchmarks are not designed for comparison to other matrix crates, but for tracking speed-ups/regressions. Comparison benchmarks are left as an exercice to the reader.

## Want to participate?
Create a new issue or a pull request and I will check them (as soon as I can).
