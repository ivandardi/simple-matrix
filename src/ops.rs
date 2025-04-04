//! The `ops` module contains traits and implementations for matrix operations.
//! It includes traits for transposition, dot product, and common matrix operations.

mod dot;
mod std_ops;
mod transpose;

pub use dot::DotProduct;
pub use transpose::Transpose;
