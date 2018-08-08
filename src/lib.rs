/*!
# Matrix-rs: A generic matrix library in Rust
Matrix-rs is a simple matrix library designed to be easy to use.

# Usage
This crate **is not** on crates.io and must be copied to local storage.  
Then, link it in your project's `Cargo.toml` file:
```toml
# Example Cargo.toml (replace values with your own)

[dependencies]
matrix-rs = { path = "path/to/matrix-rs" }
```

# Example: Basic matrix usage
```text
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

# Example: Dot product
```text
let mat: Matrix<usize> = Matrix::from_iter(2, 4, 0..);

// Construct the transposed matrix
let mat_t = mat.transpose();

// Construct the dot product
let dot = mat * mat_t;
```
*/

#![deny(missing_docs)]

mod matrix;
mod tests;

pub use crate::matrix::*;
