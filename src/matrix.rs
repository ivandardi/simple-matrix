#[cfg(feature = "impl_from")]
mod from;
mod iter;
mod std_ops;

use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Constructs a new, non-empty Matrix<T> where values are set to `T::default`.  
    /// Use `Matrix::from_iter` if you want to set the matrix from an iterator.
    /// # Panics
    /// Panics if either `rows` or `cols` are equal to `0`
    pub fn new(rows: usize, cols: usize) -> Matrix<T> 
    where
        T: Default {
        Matrix::from_iter(rows, cols, (0..).map(|_| T::default()))
    }

    /// Constructs a new, non-empty Matrix<T> where values are set from an iterator.  
    /// The matrix values are set row by row.
    /// # Panics
    /// Panics if either `rows` or `cols` are equal to `0`
    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            rows,
            cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[col + row * self.cols])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[col + row * self.cols])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
            true
        } else {
            false
        }
    }

    pub fn row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows {
            Some((0..self.cols).map(move |col| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.cols {
            Some((0..self.rows).map(move |row| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn transpose(&self) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);
                for row in 0..self.rows {
                    for val in self.row(row).unwrap() {
                        data.push(val.clone());
                    }
                }
                data
            },
        }
    }

    pub fn apply<F: Fn(&mut T)>(&mut self, func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
