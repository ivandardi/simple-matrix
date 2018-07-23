// Add implementation

use matrix::Matrix;
use std::ops::Add;
use std::ops::Sub;

// PartialEq implementation

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.rows == rhs.rows
            && self.cols == rhs.cols
            && self.into_iter().zip(rhs.into_iter()).all(|(a, b)| *a == *b)
    }
}

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
