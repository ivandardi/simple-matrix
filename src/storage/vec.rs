use crate::storage::Storage;
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

/// Vector-based storage implementation for `Storage` trait
#[derive(Clone)]
pub struct StorageVec<T> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) data: Vec<T>,
}

impl<T> StorageVec<T> {
    /// Creates a new vector storage with the specified dimensions
    ///
    /// # Panics
    /// Panics if rows or cols is 0
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default,
    {
        assert!(rows > 0 && cols > 0);
        let size = rows * cols;
        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(T::default());
        }

        Self { rows, cols, data }
    }

    /// Creates a new iterator over the storage vector
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Creates a new mutable iterator over the storage vector
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for StorageVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageVec")
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .field("data", &self.data)
            .finish()
    }
}

impl<T> Storage<T> for StorageVec<T> {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn get(&self, row: impl Into<usize>, col: impl Into<usize>) -> Option<&T> {
        let row = row.into();
        let col = col.into();

        if row >= self.rows || col >= self.cols {
            return None;
        }

        let idx = row * self.cols + col;
        self.data.get(idx)
    }

    fn get_mut(&mut self, row: impl Into<usize>, col: impl Into<usize>) -> Option<&mut T> {
        let row = row.into();
        let col = col.into();

        if row >= self.rows || col >= self.cols {
            return None;
        }

        let idx = row * self.cols + col;
        self.data.get_mut(idx)
    }

    fn set(&mut self, row: impl Into<usize>, col: impl Into<usize>, value: T) -> Option<T> {
        let row = row.into();
        let col = col.into();

        if row >= self.rows || col >= self.cols {
            return None;
        }

        let index = row * self.cols + col;
        Some(std::mem::replace(&mut self.data[index], value))
    }

    fn get_row(&self, row: impl Into<usize>) -> Option<Vec<&T>> {
        let row = row.into();
        if row >= self.rows {
            return None;
        }

        Some(
            self.data
                .iter()
                .skip(row * self.cols)
                .take(self.cols)
                .collect::<Vec<_>>(),
        )
    }

    fn get_col(&self, col: impl Into<usize>) -> Option<Vec<&T>> {
        let col = col.into();
        if col >= self.cols {
            return None;
        }

        Some(
            self.data
                .iter()
                .skip(col)
                .step_by(self.cols)
                .take(self.rows)
                .collect::<Vec<_>>(),
        )
    }
}

impl<T> IntoIterator for StorageVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a StorageVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut StorageVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
