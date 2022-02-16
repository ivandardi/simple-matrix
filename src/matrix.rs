#[cfg(feature = "impl_from")]
mod from;
mod iter;
mod std_ops;

use std::ops::{Deref, Index, IndexMut};

use itertools::Itertools;

const STRAIGHTS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const DIAGONALS: [(isize, isize); 4] = [(-1, -1), (1, 1), (-1, 1), (1, -1)];

/// A struct used to represent an adjacent location and value in the matrix.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Neighbor<'a, T> {
    /// The location of this neighbor in the matrix
    pub loc: (usize, usize),
    /// The value of the neighbor
    pub value: &'a T,
}

/// A 2-Dimensional, non-resisable container.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
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
    /// ```
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
    /// ```
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

    /// Convenience method to construct a Matrix from a row order list of vectors.
    ///
    /// # Examples
    /// ```
    /// let mat: Matrix<usize> = Matrix::from_vecs(vec![vec![1, 2], vec![3, 4]]);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 1);
    /// assert_eq!(mat.get(0, 1).unwrap(), 2);
    /// assert_eq!(mat.get(1, 0).unwrap(), 3);
    /// assert_eq!(mat.get(1, 1).unwrap(), 4);
    pub fn from_vecs(vecs: Vec<Vec<T>>) -> Self {
        let rows = vecs.len();
        assert!(vecs.iter().map(Vec::len).all_equal());
        let cols = vecs.get(0).map(Vec::len).unwrap_or_default();

        Self::from_iter(rows, cols, vecs.into_iter().flatten())
    }

    /// Returns the number of rows in the matrix.
    ///
    /// # Examples
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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

    /// Get the neighbors for a particular point in the matrix
    /// This returns an iterator over the index and values of the neighbors.
    pub fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = Neighbor<'_, T>> + '_ {
        STRAIGHTS.iter().filter_map(move |(r_dir, c_dir)| {
            let row = row.checked_add_signed(*r_dir)?;
            let col = col.checked_add_signed(*c_dir)?;
            self.get(row, col).map(|value| Neighbor {
                loc: (row, col),
                value,
            })
        })
    }

    /// Get the neighbors for a particular point in the matrix
    /// This returns an iterator over the index and values of the neighbors.
    /// Unlike [`neighbors`](Self::neighbors), this method also includes diagonal neighbors
    pub fn neighbors_with_diagonals(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = Neighbor<'_, T>> + '_ {
        STRAIGHTS
            .iter()
            .chain(DIAGONALS.iter())
            .filter_map(move |(r_dir, c_dir)| {
                let row = row.checked_add_signed(*r_dir)?;
                let col = col.checked_add_signed(*c_dir)?;
                self.get(row, col).map(|value| Neighbor {
                    loc: (row, col),
                    value,
                })
            })
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        &self.data[col + row * self.cols]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[col + row * self.cols]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut Self::Output {
        &mut self.data[col + row * self.cols]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[col + row * self.cols]
    }
}
