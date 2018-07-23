use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, Default)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new<I>(rows: usize, cols: usize, data: I) -> Matrix<T>
    where
        T: Copy + Default,
        I: IntoIterator<Item = T>,
    {
        Matrix {
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
            rows,
            cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row + col * self.rows])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[row + col * self.rows])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row + col * self.rows] = value;
            true
        } else {
            false
        }
    }

    pub fn transpose(&self) -> Matrix<T>
    where
        T: Copy,
    {
        Matrix {
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);
                for i in 0..self.rows {
                    for j in 0..self.cols {
                        data.push(self.data[i + j * self.rows])
                    }
                }
                data
            },
            rows: self.cols,
            cols: self.rows,
        }
    }

    pub fn apply<F>(&mut self, func: F)
    where
        F: Fn(&mut T),
    {
        self.data.iter_mut().for_each(|n| func(n));
    }
}

// PartialEq implementation

impl<T> PartialEq for Matrix<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.rows == rhs.rows
            && self.cols == rhs.cols
            && self.data.iter().zip(rhs.data.iter()).all(|(a, b)| *a == *b)
    }
}

// IntoIterator implementation

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

// Add implementation

impl<T: Add<Output = T>> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Self::Output {
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(other.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl<'a: 'b, 'b, T> Add for &'a Matrix<T>
where
    &'a T: Add<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    fn add(self, other: &'b Matrix<T>) -> Self::Output {
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(other.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

// Sub implementation

impl<T: Sub<Output = T>> Sub for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, other: Matrix<T>) -> Self::Output {
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(other.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl<'a: 'b, 'b, T> Sub for &'a Matrix<T>
where
    &'a T: Sub<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    fn sub(self, other: &'b Matrix<T>) -> Self::Output {
        assert!(self.rows == other.rows);
        assert!(self.cols == other.cols);

        Matrix {
        	rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(other.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}
