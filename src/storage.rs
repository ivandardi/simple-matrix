///! Storage module for matrix operations
///! This module defines the `Storage` trait and its implementations for different storage types.
///! It includes the `StorageArray` and `StorageVec` types, which represent matrix storage in a fixed-size array and a dynamic vector, respectively.

/// Module for fixed-size array storage
pub mod array;
/// Module for dynamic vector storage
pub mod vec;

pub use array::StorageArray;
pub use vec::StorageVec;

/// A trait that defines operations for matrix storage, such as get and set elements, get columns
/// and rows, transpose, etc.
///
/// All implementations of this trait guarantee that rows and columns are positive (non-zero).
pub trait Storage<T> {
    /// Get the number of rows in the storage.
    /// This will always return a value > 0.
    fn rows(&self) -> usize;

    /// Get the number of columns in the storage.
    /// This will always return a value > 0.
    fn cols(&self) -> usize;

    /// Get a reference to the element at the given row and column.
    /// Returns None if the indices are out of bounds.
    fn get(&self, row: impl Into<usize>, col: impl Into<usize>) -> Option<&T>;

    /// Get a mutable reference to the element at the given row and column.
    /// Returns None if the indices are out of bounds.
    fn get_mut(&mut self, row: impl Into<usize>, col: impl Into<usize>) -> Option<&mut T>;

    /// Set the element at the given row and column to the given value.
    /// Returns the old value, or None if the indices are out of bounds.
    fn set(&mut self, row: impl Into<usize>, col: impl Into<usize>, value: T) -> Option<T>;

    /// Get the row at the given index.
    /// Returns None if the index is out of bounds.
    fn get_row(&self, row: impl Into<usize>) -> Option<Vec<&T>>;

    /// Get the column at the given index.
    /// Returns None if the index is out of bounds.
    fn get_col(&self, col: impl Into<usize>) -> Option<Vec<&T>>;
}
