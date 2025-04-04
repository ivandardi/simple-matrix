use crate::storage::Storage;

/// Fixed-size array storage implementation for `Storage` trait
#[derive(Clone, Debug)]
pub struct StorageArray<T, const R: usize, const C: usize> {
    pub(crate) data: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> StorageArray<T, R, C> {
    /// Creates a new array storage with default values
    ///
    /// # Panics
    /// Panics if R or C is 0
    pub fn new() -> Self
    where
        T: Default + Copy,
    {
        assert!(R > 0 && C > 0);
        Self {
            data: [[T::default(); C]; R],
        }
    }

    /// Creates a new array storage filled with values from an iterator
    ///
    /// # Panics
    /// Panics if the iterator doesn't have enough elements to fill the array
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
    where
        T: Default + Copy,
    {
        let mut result = Self::new();
        let mut iter = iter.into_iter();
        let mut count = 0;

        // The total number of elements we expect
        let total_elements = R * C;

        for r in 0..R {
            for c in 0..C {
                if let Some(value) = iter.next() {
                    result.data[r][c] = value;
                    count += 1;
                } else {
                    // If we run out of elements before filling the matrix,
                    // panic with a clear error message
                    panic!("not enough elements in iterator");
                }
            }
        }

        // Ensure exactly the right number of elements were provided
        assert_eq!(count, total_elements, "not enough elements in iterator");

        result
    }

    /// Creates a new iterator over the storage array
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|row| row.iter())
    }

    /// Creates a new mutable iterator over the storage array
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().flat_map(|row| row.iter_mut())
    }
}

impl<T, const R: usize, const C: usize> Storage<T> for StorageArray<T, R, C> {
    fn rows(&self) -> usize {
        R
    }

    fn cols(&self) -> usize {
        C
    }

    fn get(&self, row: impl Into<usize>, col: impl Into<usize>) -> Option<&T> {
        let row = row.into();
        let col = col.into();

        if row >= R || col >= C {
            return None;
        }

        Some(&self.data[row][col])
    }

    fn get_mut(&mut self, row: impl Into<usize>, col: impl Into<usize>) -> Option<&mut T> {
        let row = row.into();
        let col = col.into();

        if row >= R || col >= C {
            return None;
        }

        Some(&mut self.data[row][col])
    }

    fn set(&mut self, row: impl Into<usize>, col: impl Into<usize>, value: T) -> Option<T> {
        let row = row.into();
        let col = col.into();

        if row >= R || col >= C {
            return None;
        }

        Some(std::mem::replace(&mut self.data[row][col], value))
    }

    fn get_row(&self, row: impl Into<usize>) -> Option<Vec<&T>> {
        let row = row.into();
        if row >= R {
            return None;
        }
        Some(self.data[row].iter().collect())
    }

    fn get_col(&self, col: impl Into<usize>) -> Option<Vec<&T>> {
        let col = col.into();
        if col >= C {
            return None;
        }
        Some(self.data.iter().map(|row| &row[col]).collect())
    }
}

impl<T, const R: usize, const C: usize> IntoIterator for StorageArray<T, R, C>
where
    T: Default,
{
    type Item = T;
    type IntoIter = StorageArrayIntoIter<T, R, C>;

    fn into_iter(self) -> Self::IntoIter {
        StorageArrayIntoIter::new(self)
    }
}

impl<'a, T, const R: usize, const C: usize> IntoIterator for &'a StorageArray<T, R, C> {
    type Item = &'a T;
    type IntoIter = StorageArrayIter<'a, T, R, C>;

    fn into_iter(self) -> Self::IntoIter {
        StorageArrayIter::new(self)
    }
}

impl<'a, T, const R: usize, const C: usize> IntoIterator for &'a mut StorageArray<T, R, C> {
    type Item = &'a mut T;
    type IntoIter = StorageArrayIterMut<'a, T, R, C>;

    fn into_iter(self) -> Self::IntoIter {
        StorageArrayIterMut::new(self)
    }
}

/// Iterator for the StorageArray
/// This iterator allows you to consume the storage array and iterate over its elements in a row-major order.
pub struct StorageArrayIntoIter<T, const R: usize, const C: usize> {
    storage: StorageArray<T, R, C>,
    row: usize,
    col: usize,
}

impl<T, const R: usize, const C: usize> StorageArrayIntoIter<T, R, C> {
    /// Creates a new iterator over the storage array
    ///
    /// # Safety
    /// This iterator takes ownership of the storage array, so it is unsafe to use the original storage array after this point.
    /// The iterator will consume the storage array, so it cannot be used after this point.
    pub fn new(storage: StorageArray<T, R, C>) -> Self {
        Self {
            storage,
            row: 0,
            col: 0,
        }
    }
}

impl<T, const R: usize, const C: usize> Iterator for StorageArrayIntoIter<T, R, C>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= R {
            return None;
        }

        let item = std::mem::replace(
            self.storage
                .get_mut(self.row, self.col)
                .expect("Invalid index"),
            Default::default(),
        );
        self.col += 1;

        if self.col >= C {
            self.row += 1;
            self.col = 0;
        }

        Some(item)
    }
}

/// Iterators for the StorageArray
/// These iterators allow you to iterate over the elements of the storage array in a row-major order.
/// They provide both immutable and mutable access to the elements.
pub struct StorageArrayIter<'a, T, const R: usize, const C: usize> {
    /// A reference to the storage array
    storage: &'a StorageArray<T, R, C>,
    /// The current row index
    row: usize,
    /// The current column index
    col: usize,
}

impl<'a, T, const R: usize, const C: usize> StorageArrayIter<'a, T, R, C> {
    /// Creates a new iterator over the storage array
    ///
    /// # Safety
    /// This iterator borrows the storage array, so it is unsafe to use the original storage array after this point.
    pub fn new(data: &'a StorageArray<T, R, C>) -> Self {
        Self {
            storage: data,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T, const R: usize, const C: usize> Iterator for StorageArrayIter<'a, T, R, C> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= R {
            return None;
        }

        let item = self.storage.get(self.row, self.col).unwrap();
        self.col += 1;

        if self.col >= C {
            self.row += 1;
            self.col = 0;
        }

        Some(item)
    }
}

/// Mutable iterator for the StorageArray
/// This iterator allows you to mutably iterate over the elements of the storage array in a row-major order.
pub struct StorageArrayIterMut<'a, T, const R: usize, const C: usize> {
    storage: &'a mut StorageArray<T, R, C>,
    row: usize,
    col: usize,
}

impl<'a, T, const R: usize, const C: usize> StorageArrayIterMut<'a, T, R, C> {
    /// Creates a new mutable iterator over the storage array
    pub fn new(data: &'a mut StorageArray<T, R, C>) -> Self {
        Self {
            storage: data,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T, const R: usize, const C: usize> Iterator for StorageArrayIterMut<'a, T, R, C> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= R {
            return None;
        }

        // This is tricky because we need to return references to elements without using the same
        // borrowing path twice. We'll use raw pointers to accomplish this safely.
        let ptr = &mut self.storage.data[self.row][self.col] as *mut T;
        self.col += 1;

        if self.col >= C {
            self.row += 1;
            self.col = 0;
        }

        // Safety: This is safe because we're ensuring the pointer is valid and
        // we never return the same reference twice
        Some(unsafe { &mut *ptr })
    }
}
