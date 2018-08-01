use super::Matrix;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;

macro_rules! impl_op_basic {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T: $trait<Output = T>> $trait for Matrix<T> {
            type Output = Matrix<T>;

            fn $func(self, rhs: Self) -> Self::Output {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                Matrix {
                    rows: self.rows,
                    cols: self.cols,
                    data: self
                        .into_iter()
                        .zip(rhs.into_iter())
                        .map(|(a, b)| a $op b)
                        .collect(),
                }
            }
        }

        impl<'a: 'b, 'b, T> $trait for &'a Matrix<T>
        where
            &'a T: $trait<&'b T, Output = T>,
        {
            type Output = Matrix<T>;

            fn $func(self, rhs: &'b Matrix<T>) -> Self::Output {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                Matrix {
                    rows: self.rows,
                    cols: self.cols,
                    data: self
                        .iter()
                        .zip(rhs.iter())
                        .map(|(a, b)| a $op b)
                        .collect(),
                }
            }
        }
    }
}

macro_rules! impl_op_assign_basic {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T: $trait> $trait for Matrix<T> {
            fn $func(&mut self, rhs: Self) {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                self.data.iter_mut()
                    .zip(rhs.into_iter())
                    .for_each(|(a, b)| *a $op b);
            }
        }

        impl<'a, T: $trait<&'a T>> $trait<&'a Matrix<T>> for Matrix<T> {
            fn $func(&mut self, rhs: &'a Self) {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                self.data.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(a, b)| *a $op b);
            }
        }
    }
}

macro_rules! impl_op {
    ($trait:ident, $($more:ident),*) => {
        impl_op!($trait);
        impl_op!($($more),*);
    };

    (Add) => { impl_op_basic!(Add, add, +); };
    (Sub) => { impl_op_basic!(Sub, sub, -); };
    (AddAssign) => { impl_op_assign_basic!(AddAssign, add_assign, +=); };
    (SubAssign) => { impl_op_assign_basic!(SubAssign, sub_assign, -=); };
}

// PartialEq implementation

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.rows == rhs.rows
            && self.cols == rhs.cols
            && self.iter().zip(rhs.iter()).all(|(a, b)| *a == *b)
    }
}

// Macro-ed impl

impl_op!(Add, AddAssign, Sub, SubAssign);

// Mul implementation

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<Output = T> + AddAssign + Copy,
{
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

                        let mut iter = row.zip(col);
                        let (a, b) = iter.next().unwrap();
                        let mut acc = *a * *b;

                        for (a, b) in iter {
                            acc += *a * *b;
                        }

                        data.push(acc);
                    }
                }

                data
            },
        }
    }
}

impl<'a, 'b, T: AddAssign> Mul<&'b Matrix<T>> for &'a Matrix<T>
where
    &'a T: Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
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

                        let mut iter = row.zip(col);
                        let (a, b) = iter.next().unwrap();
                        let mut acc = a * b;

                        for (a, b) in iter {
                            acc += a * b;
                        }

                        data.push(acc);
                    }
                }

                data
            },
        }
    }
}
