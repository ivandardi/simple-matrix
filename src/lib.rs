//! # simple-matrix: A generic matrix library in Rust
//!
//! [![crates.io](https://img.shields.io/crates/v/simple-matrix.svg)](https://crates.io/crates/simple-matrix)
//! [![docs.rs](https://docs.rs/simple-matrix/badge.svg)](https://docs.rs/simple-matrix)
//!
//! This library provides a simple and efficient way to work with matrices in Rust. It is designed to be easy to use,
//! flexible, and efficient, making it suitable for simple applications and educational purposes.
//!
//! ## Disclaimer
//! This crate should not be considered mature enough for *professional use*, check alternatives like [cgmath](https://github.com/brendanzab/cgmath) or [nalgebra](https://github.com/sebcrozet/nalgebra) if you are in that case.
//!
//! If you are still interested, feel free to continue!
//!
//! ## Usage
//! Link it in your project's `Cargo.toml` file:
//! ```toml
//! # Example Cargo.toml
//!
//! [dependencies]
//! simple-matrix = "0.2"
//! ```
//!
//! Then, you can use it in your project:
//! ```rust
//! // You can now use it
//! use simple_matrix::MatrixVec;
//!
//! let mat: MatrixVec<i32> = MatrixVec::new(2, 3);
//! ```
//!
//! ### Example: Basic matrix usage
//! ```rust
//! use simple_matrix::MatrixVec;
//!
//! // Create a 2x3 matrix from a vector of values
//! let mat1 = MatrixVec::from_iter(2, 3, vec![1, 2, 3, 4, 5, 6]);
//!
//! // Create another matrix with different values
//! let mat2 = MatrixVec::from_iter(2, 3, vec![7, 8, 9, 10, 11, 12]);
//!
//! // Matrix addition
//! let sum = &mat1 + &mat2;
//!
//! // Accessing elements
//! println!("mat1[0,0] = {}", mat1[[0, 0]]);
//! println!("mat1[0,1] = {}", mat1[(0, 1)]);
//! println!("mat1[1,2] = {}", mat1.get(1, 2).unwrap());
//!
//! // Iterate through the matrix (row by row)
//! for val in &mat1 {
//!     print!("{} ", val);
//! }
//!
//! // Iterator combinators
//! let sum: i32 = mat1.iter().map(|&x| x * 2).sum();
//! println!("Sum of doubled elements: {}", sum);
//! ```
//!
//! ### Example: Dot product and Transposition
//! ```rust
//! use simple_matrix::MatrixVec;
//! use simple_matrix::ops::{Transpose, DotProduct};
//!
//! // Create a matrix from values
//! let mat = MatrixVec::from_iter(2, 3, vec![1, 2, 3, 4, 5, 6]);
//!
//! // Transpose the matrix
//! let transposed = mat.transpose();
//!
//! // Matrix multiplication (dot product)
//! let dot_product = mat.dot(&transposed);
//! ```
//!
//! ### Matrix Storage Types
//!
//! Simple-matrix supports two different storage backends:
//!
//! #### Vector-based Storage
//! The default storage type is a dynamic vector, which is flexible and allows matrices of any size:
//!
//! ```rust
//! use simple_matrix::MatrixVec;
//!
//! // Creating a vector-based matrix with 3 rows and 4 columns
//! let vec_matrix = MatrixVec::<i32>::new(3, 4);
//!
//! // Create a matrix from specific values
//! let matrix = MatrixVec::from_iter(2, 3, vec![1, 2, 3, 4, 5, 6]);
//! ```
//!
//! #### Array-based Storage
//! For fixed-size matrices known at compile time, you can use array-based storage:
//!
//! ```rust
//! use simple_matrix::MatrixArray;
//!
//! // Creating an array-based matrix with fixed dimensions
//! let mut arr_matrix = MatrixArray::<i32, 2, 3>::new();
//!
//! // Set values manually
//! arr_matrix[[0, 0]] = 1;
//! arr_matrix[[0, 1]] = 2;
//! arr_matrix[[0, 2]] = 3;
//! arr_matrix[[1, 0]] = 4;
//! arr_matrix[[1, 1]] = 5;
//! arr_matrix[[1, 2]] = 6;
//! ```
//!
//! ### Advanced Examples
//!
//! #### Creating and Using an Identity Matrix
//! ```rust
//! use simple_matrix::{MatrixVec, MatrixArray};
//! use simple_matrix::ops::DotProduct;
//!
//! // Create an identity matrix using vector storage
//! fn create_identity_vec(size: usize) -> MatrixVec<i32> {
//!     let mut identity = MatrixVec::<i32>::identity(size, 1);
//!     identity
//! }
//!
//! // For array-based identity matrices
//! fn create_identity_array<const N: usize>() -> MatrixArray<i32, N, N> {
//!     MatrixArray::<i32, N, N>::identity(1)
//! }
//!
//! // Using the identity matrix
//! let matrix = MatrixVec::from_iter(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
//! let identity = create_identity_vec(3);
//! let result = matrix.dot(&identity); // Should be equal to original matrix
//! ```
//!
//! #### Working with Matrix Statistics
//! ```rust
//! use simple_matrix::MatrixVec;
//!
//! // Create a sample matrix
//! let matrix = MatrixVec::from_iter(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
//!
//! // Sum of all elements using iteration
//! let sum: i32 = matrix.iter().map(|&x| x).sum();
//!
//! // Diagonal elements
//! for i in 0..matrix.rows().min(matrix.cols()) {
//!     let diagonal_element = matrix[[i, i]];
//!     // Do something with diagonal_element
//! }
//!
//! // Trace (sum of diagonal elements)
//! let trace: i32 = (0..matrix.rows().min(matrix.cols()))
//!     .map(|i| matrix[[i, i]])
//!     .sum();
//! ```
//!
//! ### Supported Operations
//!
//! The library supports a full set of element-wise operations for working with matrices:
//!
//! #### Matrix-Matrix Operations
//! - Addition: `matrix1 + matrix2`
//! - Subtraction: `matrix1 - matrix2`
//! - Element-wise multiplication: `matrix1 * matrix2`
//! - Element-wise division: `matrix1 / matrix2`
//! - Element-wise remainder: `matrix1 % matrix2`
//! - Bitwise operations: `&`, `|`, `^`, `<<`, `>>`
//!
//! #### Matrix-Scalar Operations
//! For operations with scalars, use the iter and iter_mut methods:
//! ```rust
//! use simple_matrix::MatrixVec;
//!
//! // Create a sample matrix
//! let mut matrix = MatrixVec::from_iter(2, 2, vec![1, 2, 3, 4]);
//!
//! // Multiply each element by 2
//! let doubled = matrix.clone().iter_mut().for_each(|val| *val *= 2);
//!
//! // Bitshift each element left by 3
//! for val in matrix.iter_mut() {
//!     *val <<= 3;
//! }
//! ```
//!
//! #### Assignment Operations
//! - Matrix-Matrix: `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`
//!
//! #### Matrix Functions
//! - `transpose()` - Creates a new matrix with rows and columns swapped
//! - `dot()` - Matrix multiplication (dot product)
//! - `iter()` - Returns an iterator over references to all elements in the matrix
//! - `iter_mut()` - Returns an iterator over mutable references to all elements in the matrix
//!
//! #### Element Access
//! - `get(row, col)` - Get a reference to an element
//! - `get_mut(row, col)` - Get a mutable reference to an element
//! - `set(row, col, value)` - Set an element to a new value
//! - `matrix[(row, col)]` - Index-based access (tuple)
//! - `matrix[[row, col]]` - Index-based access (array)
//! - `get_row(row)`, `get_col(col)` - Get entire rows and columns
//!
//! #### Iteration
//! - `for element in matrix` - Iterate through all elements row by row
//! - `for element in &matrix` - Iterate through references to elements
//! - `for element in &mut matrix` - Iterate through mutable references

#![deny(missing_docs)]

mod matrix;
pub mod ops;
pub mod storage;

pub use crate::matrix::*;
