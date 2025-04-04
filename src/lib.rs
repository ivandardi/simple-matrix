//! # simple-matrix: A generic matrix library in Rust

#![deny(missing_docs)]

mod matrix;
/// Module containing traits and implementations for matrix operations
pub mod ops;
/// Module containing storage implementations for matrices
pub mod storage;

pub use crate::matrix::*;
