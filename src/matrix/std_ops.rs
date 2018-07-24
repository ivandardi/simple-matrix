use matrix::Matrix;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;

// PartialEq implementation

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.rows == rhs.rows
            && self.cols == rhs.cols
            && self.into_iter().zip(rhs.into_iter()).all(|(a, b)| *a == *b)
    }
}

// AddAssign implementation

impl<T: AddAssign> AddAssign<Matrix<T>> for Matrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        self.into_iter()
            .zip(rhs.into_iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl<'a, T: AddAssign<&'a T>> AddAssign<&'a Matrix<T>> for Matrix<T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        self.into_iter()
            .zip(rhs.into_iter())
            .for_each(|(a, b)| *a += b);
    }
}

// Add implementation

impl<T: Add<Output = T>> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(rhs.into_iter())
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

    fn add(self, rhs: &'b Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(rhs.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

// SubAssign implementation

impl<T: SubAssign> SubAssign<Matrix<T>> for Matrix<T> {
    fn sub_assign(&mut self, rhs: Self) {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        self.into_iter()
            .zip(rhs.into_iter())
            .for_each(|(a, b)| *a -= b);
    }
}

impl<'a, T: SubAssign<&'a T>> SubAssign<&'a Matrix<T>> for Matrix<T> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        self.into_iter()
            .zip(rhs.into_iter())
            .for_each(|(a, b)| *a -= b);
    }
}

// Sub implementation

impl<T: Sub<Output = T>> Sub for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(rhs.into_iter())
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

    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(rhs.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

// Mul implementation

impl<T: Mul<Output = T> + Add<Output = T> + Default + Copy> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.cols == rhs.rows);

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * rhs.cols);

                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        let row = self.row(row).unwrap();
                        let col = rhs.col(col).unwrap();

                        data.push(
                            row.zip(col)
                                .fold(T::default(), |acc, (a, b)| acc + (*a * *b)),
                        );
                    }
                }

                data
            },
        }
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.into_iter().map(|n| n * rhs.clone()).collect(),
        }
    }
}
