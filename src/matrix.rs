use crate::storage::{Storage, StorageArray, StorageVec};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// A 2-Dimensional, non-resizable matrix container. The elements are stored in row-major order.
///
/// The `Matrix` struct is generic over a type `T` and a storage type `S` that implements the `Storage<T>` trait.
/// This allows for different storage backends while providing a consistent interface.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T, S>
where
    S: Storage<T>,
{
    data: S,
    _storage_type: PhantomData<T>,
}

/// Type alias for a matrix with a fixed size array as storage.
///
/// `R` represents the number of rows and `C` represents the number of columns.
pub type MatrixArray<T, const R: usize, const C: usize> = Matrix<T, StorageArray<T, R, C>>;

/// Type alias for a matrix with a vector as storage.
///
/// This type provides a dynamically sized matrix that's allocated on the heap.
pub type MatrixVec<T> = Matrix<T, StorageVec<T>>;

impl<T> MatrixVec<T>
where
    T: Default,
{
    /// Creates a new vector-backed matrix with the specified dimensions.
    ///
    /// # Parameters
    /// * `rows` - The number of rows in the matrix
    /// * `cols` - The number of columns in the matrix
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// // Create a 2x3 vector-backed matrix
    /// let matrix = MatrixVec::<i32>::new(2, 3);
    /// assert_eq!(matrix.rows(), 2);
    /// assert_eq!(matrix.cols(), 3);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: StorageVec::new(rows, cols),
            _storage_type: PhantomData,
        }
    }

    /// Creates a new vector-backed matrix from an iterator.
    ///
    /// Elements are filled in row-major order (row by row, from left to right).
    ///
    /// # Parameters
    /// * `rows` - The number of rows in the matrix
    /// * `cols` - The number of columns in the matrix
    /// * `iter` - An iterator yielding elements to fill the matrix
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// // Create a 2x3 matrix with values 0 through 5
    /// let matrix = MatrixVec::from_iter(2, 3, 0..6);
    /// assert_eq!(matrix[[0, 0]], 0);
    /// assert_eq!(matrix[[1, 2]], 5);
    /// ```
    ///
    /// # Panics
    /// Panics if the iterator doesn't contain enough elements to fill the matrix or if
    /// `rows` or `cols` are zero.
    pub fn from_iter<I>(rows: usize, cols: usize, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut data = StorageVec::new(rows, cols);
        let mut iter = iter.into_iter();
        let expected_elements = rows * cols;
        let mut count = 0;

        for r in 0..rows {
            for c in 0..cols {
                match iter.next() {
                    Some(value) => {
                        data.set(r, c, value).unwrap();
                        count += 1;
                    }
                    None => {
                        panic!("not enough elements in iterator");
                    }
                }
            }
        }

        // Verify we got the expected number of elements
        assert_eq!(count, expected_elements, "not enough elements in iterator");

        Matrix {
            data,
            _storage_type: PhantomData,
        }
    }

    /// Creates a new identity matrix of the specified size.
    ///
    /// An identity matrix is a square matrix with the specified value on the main diagonal and zeros elsewhere.
    /// This method creates a square matrix with size × size dimensions.
    ///
    /// # Parameters
    /// * `size` - The number of rows and columns in the matrix
    /// * `elem` - The value to place on the diagonal (typically 1 for a standard identity matrix)
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// // Create a 3x3 identity matrix with 1s on the diagonal
    /// let identity_matrix = MatrixVec::<i32>::identity(3, 1);
    ///
    /// // Diagonal elements should be 1, others should be 0
    /// assert_eq!(identity_matrix[[0, 0]], 1);
    /// assert_eq!(identity_matrix[[1, 1]], 1);
    /// assert_eq!(identity_matrix[[2, 2]], 1);
    /// assert_eq!(identity_matrix[[0, 1]], 0);
    /// assert_eq!(identity_matrix[[1, 0]], 0);
    /// ```
    ///
    /// # Panics
    /// Panics if `size` is zero.
    pub fn identity(size: usize, elem: T) -> Self
    where
        T: Default + Copy + PartialEq,
    {
        let mut data = StorageVec::new(size, size);

        for i in 0..size {
            for j in 0..size {
                if i == j {
                    data.set(i, j, elem).unwrap();
                } else {
                    data.set(i, j, T::default()).unwrap();
                }
            }
        }

        Matrix {
            data,
            _storage_type: PhantomData,
        }
    }
}

impl<T, const R: usize, const C: usize> MatrixArray<T, R, C>
where
    T: Default + Copy,
{
    /// Creates a new array-backed matrix with default values.
    ///
    /// The dimensions are specified by the const generic parameters `R` (rows) and `C` (columns).
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixArray;
    ///
    /// // Create a 2x3 array-backed matrix
    /// let matrix = MatrixArray::<i32, 2, 3>::new();
    /// assert_eq!(matrix.rows(), 2);
    /// assert_eq!(matrix.cols(), 3);
    /// ```
    pub fn new() -> Self {
        Matrix {
            data: StorageArray::new(),
            _storage_type: PhantomData,
        }
    }

    /// Creates a new array-backed matrix filled with values from an iterator.
    ///
    /// Elements are filled in row-major order (row by row, from left to right).
    ///
    /// # Parameters
    /// * `iter` - An iterator yielding elements to fill the matrix
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixArray;
    ///
    /// // Create a 2x3 matrix with values 0 through 5
    /// let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);
    /// assert_eq!(matrix[[0, 0]], 0);
    /// assert_eq!(matrix[[1, 2]], 5);
    /// ```
    ///
    /// # Panics
    /// Panics if the iterator doesn't contain enough elements to fill the matrix.
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut data = StorageArray::new();
        let mut iter = iter.into_iter();
        let mut count = 0;
        let total = R * C;

        for r in 0..R {
            for c in 0..C {
                match iter.next() {
                    Some(value) => {
                        data.set(r, c, value).unwrap();
                        count += 1;
                    }
                    None => {
                        panic!("not enough elements in iterator");
                    }
                }
            }
        }

        // Double-check that we got exactly the right number of elements
        assert_eq!(count, total, "not enough elements in iterator");

        Matrix {
            data,
            _storage_type: PhantomData,
        }
    }

    /// Creates a new identity matrix.
    ///
    /// An identity matrix is a square matrix with the specified value on the main diagonal and zeros elsewhere.
    /// This method is only available for square matrices where R equals C.
    ///
    /// # Parameters
    /// * `elem` - The value to place on the diagonal (typically 1 for a standard identity matrix)
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixArray;
    ///
    /// // Create a 3x3 identity matrix with 1s on the diagonal
    /// let identity_matrix = MatrixArray::<i32, 3, 3>::identity(1);
    ///
    /// // Diagonal elements should be 1, others should be 0
    /// assert_eq!(identity_matrix[[0, 0]], 1);
    /// assert_eq!(identity_matrix[[1, 1]], 1);
    /// assert_eq!(identity_matrix[[2, 2]], 1);
    /// assert_eq!(identity_matrix[[0, 1]], 0);
    /// assert_eq!(identity_matrix[[1, 0]], 0);
    /// ```
    ///
    /// # Panics
    /// Panics if the matrix is not square (R != C).
    pub fn identity(elem: T) -> Self
    where
        T: Default + Copy + PartialEq,
    {
        // Check that the matrix is square
        assert_eq!(R, C, "dimensions must match for an identity matrix");

        let mut data = StorageArray::new();

        for i in 0..R {
            for j in 0..C {
                if i == j {
                    data.set(i, j, elem).unwrap();
                } else {
                    data.set(i, j, T::default()).unwrap();
                }
            }
        }

        Matrix {
            data,
            _storage_type: PhantomData,
        }
    }
}

impl<T, S> Matrix<T, S>
where
    S: Storage<T>,
{
    /// Returns the number of rows in the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mat = MatrixVec::<i32>::new(3, 6);
    /// assert_eq!(mat.rows(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        self.data.rows()
    }

    /// Returns the number of columns in the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mat = MatrixVec::<i32>::new(3, 6);
    /// assert_eq!(mat.cols(), 6);
    /// ```
    pub fn cols(&self) -> usize {
        self.data.cols()
    }

    /// Gets a reference to the value at given row and column.
    ///
    /// # Parameters
    /// * `row` - The row index (0-based)
    /// * `col` - The column index (0-based)
    ///
    /// # Returns
    /// * `Some(&T)` - A reference to the value if the indices are valid
    /// * `None` - If the row or column is outside the matrix boundaries
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mat = MatrixVec::from_iter(3, 6, 0..18);
    /// assert_eq!(mat.get(0, 0), Some(&0));
    /// assert_eq!(mat.get(2, 5), Some(&17));
    /// assert_eq!(mat.get(3, 0), None);  // Out of bounds
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row, col)
    }

    /// Gets a mutable reference to the value at the given row and column.
    ///
    /// # Parameters
    /// * `row` - The row index (0-based)
    /// * `col` - The column index (0-based)
    ///
    /// # Returns
    /// * `Some(&mut T)` - A mutable reference to the value if the indices are valid
    /// * `None` - If the row or column is outside the matrix boundaries
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mut mat = MatrixVec::from_iter(3, 6, 0..18);
    /// if let Some(val) = mat.get_mut(0, 0) {
    ///     *val = 100;
    /// }
    /// assert_eq!(mat.get(0, 0), Some(&100));
    /// assert_eq!(mat.get_mut(3, 0), None);  // Out of bounds
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row, col)
    }

    /// Sets the cell at the given row and column to the specified value.
    ///
    /// # Parameters
    /// * `row` - The row index (0-based)
    /// * `col` - The column index (0-based)
    /// * `value` - The new value to set
    ///
    /// # Returns
    /// * `Some(T)` - The previous value if the indices are valid
    /// * `None` - If the row or column is outside the matrix boundaries
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mut mat = MatrixVec::from_iter(3, 6, 0..18);
    /// assert_eq!(mat.set(1, 2, 42), Some(8));
    /// assert_eq!(mat.get(1, 2), Some(&42));
    /// assert_eq!(mat.set(3, 0, 0), None);  // Out of bounds
    /// ```
    pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<T> {
        self.data.set(row, col, value)
    }

    /// Gets an iterator over all elements in the requested row.
    ///
    /// # Parameters
    /// * `row` - The row index (0-based)
    ///
    /// # Returns
    /// * `Some(Iterator)` - An iterator yielding references to the elements if the row is valid
    /// * `None` - If the row is outside the matrix boundaries
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mat = MatrixVec::from_iter(3, 6, 0..18);
    /// if let Some(row_iter) = mat.get_row(1) {
    ///     let row_vec: Vec<&i32> = row_iter.into_iter().collect();
    ///     assert_eq!(row_vec, vec![&6, &7, &8, &9, &10, &11]);
    /// }
    /// assert!(mat.get_row(10).is_none());  // Out of bounds
    /// ```
    pub fn get_row(&self, row: usize) -> Option<impl IntoIterator<Item = &T>> {
        self.data.get_row(row)
    }

    /// Gets an iterator over all elements in the requested column.
    ///
    /// # Parameters
    /// * `col` - The column index (0-based)
    ///
    /// # Returns
    /// * `Some(Iterator)` - An iterator yielding references to the elements if the column is valid
    /// * `None` - If the column is outside the matrix boundaries
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mat = MatrixVec::from_iter(3, 6, 0..18);
    /// if let Some(col_iter) = mat.get_col(1) {
    ///     let col_vec: Vec<&i32> = col_iter.into_iter().collect();
    ///     assert_eq!(col_vec, vec![&1, &7, &13]);
    /// }
    /// assert!(mat.get_col(10).is_none());  // Out of bounds
    /// ```
    pub fn get_col(&self, col: usize) -> Option<impl IntoIterator<Item = &T>> {
        self.data.get_col(col)
    }
}

impl<T, S> Matrix<T, S>
where
    S: Storage<T>,
    for<'a> &'a S: IntoIterator<Item = &'a T>,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut T>,
{
    /// Returns an iterator over all elements of the matrix in row-major order.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mat = MatrixVec::from_iter(2, 2, 0..4);
    /// let elements: Vec<&i32> = mat.iter().collect();
    /// assert_eq!(elements, vec![&0, &1, &2, &3]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.into_iter()
    }

    /// Returns a mutable iterator over all elements of the matrix in row-major order.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::MatrixVec;
    ///
    /// let mut mat = MatrixVec::from_iter(2, 2, 0..4);
    /// for val in mat.iter_mut() {
    ///     *val += 10;
    /// }
    /// assert_eq!(mat[[0, 0]], 10);
    /// assert_eq!(mat[[1, 1]], 13);
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        (&mut self.data).into_iter()
    }
}

impl<T, S> Index<[usize; 2]> for Matrix<T, S>
where
    S: Storage<T>,
{
    type Output = T;

    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        &self.data.get(row, col).expect("Invalid index")
    }
}

impl<T, S> Index<(usize, usize)> for Matrix<T, S>
where
    S: Storage<T>,
{
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data.get(row, col).expect("Invalid index")
    }
}

impl<T, S> IndexMut<[usize; 2]> for Matrix<T, S>
where
    S: Storage<T>,
{
    fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut Self::Output {
        self.data.get_mut(row, col).expect("Invalid index")
    }
}

impl<T, S> IndexMut<(usize, usize)> for Matrix<T, S>
where
    S: Storage<T>,
{
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.data.get_mut(row, col).expect("Invalid index")
    }
}

impl<T, S> IntoIterator for Matrix<T, S>
where
    S: Storage<T> + IntoIterator,
{
    type Item = <S as IntoIterator>::Item;
    type IntoIter = <S as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, S> IntoIterator for &'a Matrix<T, S>
where
    &'a S: IntoIterator,
    S: Storage<T>,
{
    type Item = <&'a S as IntoIterator>::Item;
    type IntoIter = <&'a S as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, S> IntoIterator for &'a mut Matrix<T, S>
where
    &'a mut S: IntoIterator,
    S: Storage<T>,
{
    type Item = <&'a mut S as IntoIterator>::Item;
    type IntoIter = <&'a mut S as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
