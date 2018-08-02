#[cfg(feature = "impl_from")]
mod from;
mod iter;
mod std_ops;

use std::ops::Deref;

/// A 2-Dimensional, non-resisable container.
#[derive(Clone, Debug, Hash, Eq, PartialOrd)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Constructs a new, non-empty Matrix<T> where cells are set to `T::default`.  
    /// Use `Matrix::from_iter` if you want to set the matrix from an iterator.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are equal to `0`
    ///
    /// # Examples
    /// ```text
    /// let mut mat: Matrix<i32> = Matrix::new(3, 6);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Matrix<T>
    where
        T: Default,
    {
        Matrix::from_iter(rows, cols, (0..).map(|_| T::default()))
    }

    /// Constructs a new, non-empty Matrix<T> where cells are set from an iterator.  
    /// The matrix cells are set row by row.  
    /// The iterator can be infinite, this method only consume `rows * cols`
    /// values from the iterator.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are equal to `0`.  
    /// Panics if the iterator does not have `rows * cols` values
    ///
    /// # Examples
    /// ```text
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    /// assert_eq!(mat.get(0, 1).unwrap(), 1);
    /// assert_eq!(mat.get(1, 0).unwrap(), 6);
    /// ```
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

    /// Returns the number of rows in the matrix.
    ///
    /// # Examples
    /// ```text
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// 
    /// assert_eq!(mat.rows(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix.
    ///
    /// # Examples
    /// ```text
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// 
    /// assert_eq!(mat.cols(), 6);
    /// ```
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Try to get a reference to the value at given row & column.  
    /// Returns `None` if `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```text
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// 
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    /// assert_eq!(mat.get(2, 5).unwrap(), 17);
    ///
    /// assert!(mat.get(10, 2).is_err());
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Try to get a mutable reference to the cell at given row & column.  
    /// Returns `None` if `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```text
    /// let mut mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    ///
    /// let cell = mat.get_mut(0, 0).unwrap();
    /// *cell = 5;
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 5);
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Try to set the cell at given row & column to the given value.  
    /// Returns `false` if `row` or `col` is outside of the matrix.  
    /// Returns `true` if the cell has been modified.
    ///
    /// # Examples
    /// ```text
    /// let mut mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    ///
    /// mat.set(0, 0, 5);
    /// assert_eq!(mat.get(0, 0).unwrap(), 5);
    /// ```
    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
            true
        } else {
            false
        }
    }

    /// Try to get an iterator of all cells of the requested row.  
    /// Returns `None` if given row is outside of the matrix.
    ///
    /// # Examples
    /// ```text
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// 
    /// assert_eq!(mat.get_row(1).unwrap(), vec![6, 7, 8, 9, 10, 11]);
    ///
    /// assert!(mat.get_row(5).is_err());
    /// ```
    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows {
            Some((0..self.cols).map(move |col| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    /// Try to get an iterator of all cells of the requested column.  
    /// Returns `None` if given row is outside of the matrix.
    ///
    /// # Examples
    /// ```text
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// 
    /// assert_eq!(mat.get_col(1).unwrap(), vec![1, 7, 13]);
    ///
    /// assert!(mat.get_col(10).is_err());
    /// ```
    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.cols {
            Some((0..self.rows).map(move |row| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    /// Take a *M*x*N* Matrix and construct the transposed *N*x*M* Matrix.
    ///
    /// # Examples
    /// ```text
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// let mat_t = mat.transpose();
    ///
    /// assert_eq!(mat.rows(), mat_t.cols());
    /// assert_eq!(mat.cols(), mat_t.rows());
    /// 
    /// assert_eq!(mat.get(0, 0).unwrap(), mat_t.get(0, 0).unwrap());
    /// assert_eq!(mat.get(1, 2).unwrap(), mat_t.get(2, 1).unwrap());
    /// ```
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
                    for val in self.get_row(row).unwrap() {
                        data.push(val.clone());
                    }
                }
                data
            },
        }
    }

    /// Apply a function to all cells of the matrix.  
    /// Cells are provided as immutable references to the function,
    /// if you want to modify the cells, use `apply_mut`.
    ///
    /// # Examples
    /// ```text
    /// // Get the sum of all cells
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// let mut sum = 0;
    /// mat.apply(|n| sum += *n);
    ///
    /// assert_eq!(sum, 153);
    /// ```
    pub fn apply<F: FnMut(&T)>(&self, mut func: F) {
        self.data.iter().for_each(|n| func(n));
    }

    /// Apply a function to all cells of the matrix.  
    /// Cells are provided as mutable references to the function,
    /// and can therefore be modified.
    ///
    /// # Examples
    /// ```text
    /// // Modify all cells with a function
    /// let mut mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// mat.apply_mut(|n| n *= 2);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    /// assert_eq!(mat.get(0, 1).unwrap(), 2);
    /// assert_eq!(mat.get(0, 2).unwrap(), 4);
    /// ```
    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
