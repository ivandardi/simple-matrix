use crate::{MatrixArray, MatrixVec};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

// Specialized implementation for StorageVec
macro_rules! impl_op_vec {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T, U, V> $trait<MatrixVec<U>> for MatrixVec<T>
        where
            T: $trait<U, Output = V> + Clone,
            U: Clone,
            V: Default + Copy,
        {
            type Output = MatrixVec<V>;

            fn $func(self, rhs: MatrixVec<U>) -> Self::Output {
                assert_eq!(self.rows(), rhs.rows());
                assert_eq!(self.cols(), rhs.cols());

                // Store rows and cols before moving self.data
                let rows = self.rows();
                let cols = self.cols();

                // Create a new result matrix
                let mut result = MatrixVec::<V>::new(rows, cols);

                // Manually perform the operation
                for r in 0..rows {
                    for c in 0..cols {
                        result[(r, c)] = self[(r, c)].clone() $op rhs[(r, c)].clone();
                    }
                }

                result
            }
        }
    }
}

// Specialized implementation for StorageArray
macro_rules! impl_op_array {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T, U, V, const R: usize, const C: usize> $trait<MatrixArray<U, R, C>> for MatrixArray<T, R, C>
        where
            T: $trait<U, Output = V> + Clone + Default + Copy,
            U: Clone + Default + Copy,
            V: Default + Copy,
        {
            type Output = MatrixArray<V, R, C>;

            fn $func(self, rhs: MatrixArray<U, R, C>) -> Self::Output {
                // Create a new array storage
                let mut result = MatrixArray::<V, R, C>::new();

                // Manually perform element-wise operation without allocating
                for r in 0..R {
                    for c in 0..C {
                        result[(r, c)] = self[(r, c)] $op rhs[(r, c)];
                    }
                }

                result
            }
        }
    }
}

// Specialized implementation for assignment operations with StorageVec
macro_rules! impl_op_assign_vec {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T, U> $trait<MatrixVec<U>> for MatrixVec<T>
        where
            T: $trait<U> + Clone,
            U: Clone,
        {
            fn $func(&mut self, rhs: MatrixVec<U>) {
                assert_eq!(self.rows(), rhs.rows());
                assert_eq!(self.cols(), rhs.cols());

                for r in 0..self.rows() {
                    for c in 0..self.cols() {
                        *self.get_mut(r, c).unwrap() $op rhs[(r, c)].clone();
                    }
                }
            }
        }

        impl<'b, T, U> $trait<&'b MatrixVec<U>> for MatrixVec<T>
        where
            T: $trait<U> + Clone,
            U: Clone,
        {
            fn $func(&mut self, rhs: &'b MatrixVec<U>) {
                assert_eq!(self.rows(), rhs.rows());
                assert_eq!(self.cols(), rhs.cols());

                for r in 0..self.rows() {
                    for c in 0..self.cols() {
                        *self.get_mut(r, c).unwrap() $op rhs[(r, c)].clone();
                    }
                }
            }
        }
    }
}

// Specialized implementation for assignment operations with StorageArray
macro_rules! impl_op_assign_array {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T, U, const R: usize, const C: usize> $trait<MatrixArray<U, R, C>> for MatrixArray<T, R, C>
        where
            T: $trait<U> + Clone,
            U: Clone,
        {
            fn $func(&mut self, rhs: MatrixArray<U, R, C>) {
                for r in 0..R {
                    for c in 0..C {
                        *self.get_mut(r, c).unwrap() $op rhs[(r, c)].clone();
                    }
                }
            }
        }

        impl<'b, T, U, const R: usize, const C: usize> $trait<&'b MatrixArray<U, R, C>> for MatrixArray<T, R, C>
        where
            T: $trait<U> + Clone,
            U: Clone,
        {
            fn $func(&mut self, rhs: &'b MatrixArray<U, R, C>) {
                for r in 0..R {
                    for c in 0..C {
                        *self.get_mut(r, c).unwrap() $op rhs[(r, c)].clone();
                    }
                }
            }
        }
    }
}

// Specialized reference-to-reference implementation for StorageVec
macro_rules! impl_ref_op_vec {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<'a, 'b, T, U, V> $trait<&'b MatrixVec<U>> for &'a MatrixVec<T>
        where
            T: Clone,
            U: Clone,
            V: Default + Copy,
            T: $trait<U, Output = V>,
        {
            type Output = MatrixVec<V>;

            fn $func(self, rhs: &'b MatrixVec<U>) -> Self::Output {
                assert_eq!(self.rows(), rhs.rows(), "dimensions must match");
                assert_eq!(self.cols(), rhs.cols(), "dimensions must match");

                let rows = self.rows();
                let cols = self.cols();

                let mut result = MatrixVec::<V>::new(rows, cols);

                for r in 0..rows {
                    for c in 0..cols {
                        result[(r, c)] = self[(r, c)].clone() $op rhs[(r, c)].clone();
                    }
                }

                result
            }
        }
    }
}

// Handle multiplication specially for numeric types to avoid panicking on overflow
macro_rules! impl_ref_mul_vec {
    () => {
        impl<'a, 'b, T, U, V> std::ops::Mul<&'b MatrixVec<U>> for &'a MatrixVec<T>
        where
            T: Clone + std::ops::Mul<U, Output = V>,
            U: Clone,
            V: Default + Copy,
        {
            type Output = MatrixVec<V>;

            fn mul(self, rhs: &'b MatrixVec<U>) -> Self::Output {
                assert_eq!(self.rows(), rhs.rows(), "dimensions must match");
                assert_eq!(self.cols(), rhs.cols(), "dimensions must match");

                let rows = self.rows();
                let cols = self.cols();

                let mut result = MatrixVec::<V>::new(rows, cols);

                for r in 0..rows {
                    for c in 0..cols {
                        // Handle multiplication explicitly
                        result[(r, c)] = self[(r, c)].clone() * rhs[(r, c)].clone();
                    }
                }

                result
            }
        }
    };
}

// Handle division specially to check for division by zero
macro_rules! impl_ref_div_vec {
    () => {
        impl<'a, 'b, T, U, V> std::ops::Div<&'b MatrixVec<U>> for &'a MatrixVec<T>
        where
            T: Clone + std::ops::Div<U, Output = V>,
            U: Clone + Default + PartialEq,
            V: Default + Copy,
        {
            type Output = MatrixVec<V>;

            fn div(self, rhs: &'b MatrixVec<U>) -> Self::Output {
                assert_eq!(self.rows(), rhs.rows(), "dimensions must match");
                assert_eq!(self.cols(), rhs.cols(), "dimensions must match");

                let rows = self.rows();
                let cols = self.cols();

                let mut result = MatrixVec::<V>::new(rows, cols);
                let zero = U::default();

                for r in 0..rows {
                    for c in 0..cols {
                        // Check for division by zero
                        if rhs[(r, c)] == zero {
                            panic!("attempt to divide by zero");
                        }
                        result[(r, c)] = self[(r, c)].clone() / rhs[(r, c)].clone();
                    }
                }

                result
            }
        }
    };
}

// Specialized reference-to-reference implementation for StorageArray
macro_rules! impl_ref_op_array {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<'a, 'b, T, U, V, const R: usize, const C: usize>
            $trait<&'b MatrixArray<U, R, C>> for &'a MatrixArray<T, R, C>
        where
            T: Clone + Default + Copy,
            U: Clone + Default + Copy,
            V: Default + Copy,
            T: $trait<U, Output = V>,
        {
            type Output = MatrixArray<V, R, C>;

            fn $func(self, rhs: &'b MatrixArray<U, R, C>) -> Self::Output {
                let mut result = MatrixArray::<V, R, C>::new();

                for r in 0..R {
                    for c in 0..C {
                        result[(r, c)] = self[(r, c)].clone() $op rhs[(r, c)].clone();
                    }
                }

                result
            }
        }
    }
}

// Handle multiplication specially for numeric types to avoid panicking on overflow
macro_rules! impl_ref_mul_array {
    () => {
        impl<'a, 'b, T, U, V, const R: usize, const C: usize>
            std::ops::Mul<&'b MatrixArray<U, R, C>> for &'a MatrixArray<T, R, C>
        where
            T: Clone + Default + Copy + std::ops::Mul<U, Output = V>,
            U: Clone + Default + Copy,
            V: Default + Copy,
        {
            type Output = MatrixArray<V, R, C>;

            fn mul(self, rhs: &'b MatrixArray<U, R, C>) -> Self::Output {
                let mut result = MatrixArray::<V, R, C>::new();

                for r in 0..R {
                    for c in 0..C {
                        // Handle multiplication explicitly
                        result[(r, c)] = self[(r, c)].clone() * rhs[(r, c)].clone();
                    }
                }

                result
            }
        }
    };
}

// Handle division specially for array matrices
macro_rules! impl_ref_div_array {
    () => {
        impl<'a, 'b, T, U, V, const R: usize, const C: usize>
            std::ops::Div<&'b MatrixArray<U, R, C>> for &'a MatrixArray<T, R, C>
        where
            T: Clone + Default + Copy + std::ops::Div<U, Output = V>,
            U: Clone + Default + Copy + PartialEq,
            V: Default + Copy,
        {
            type Output = MatrixArray<V, R, C>;

            fn div(self, rhs: &'b MatrixArray<U, R, C>) -> Self::Output {
                let mut result = MatrixArray::<V, R, C>::new();
                let zero = U::default();

                for r in 0..R {
                    for c in 0..C {
                        // Check for division by zero
                        if rhs[(r, c)] == zero {
                            panic!("attempt to divide by zero");
                        }
                        result[(r, c)] = self[(r, c)].clone() / rhs[(r, c)].clone();
                    }
                }

                result
            }
        }
    };
}

// Implement the operations using StorageVec-specific implementations
impl_op_vec!(Add, add, +);
impl_op_vec!(Sub, sub, -);
impl_op_vec!(Mul, mul, *);
impl_op_vec!(Div, div, /);
impl_op_vec!(Rem, rem, %);
impl_op_vec!(BitAnd, bitand, &);
impl_op_vec!(BitOr, bitor, |);
impl_op_vec!(BitXor, bitxor, ^);
impl_op_vec!(Shl, shl, <<);
impl_op_vec!(Shr, shr, >>);

// Implement operations for StorageArray
impl_op_array!(Add, add, +);
impl_op_array!(Sub, sub, -);
impl_op_array!(Mul, mul, *);
impl_op_array!(Div, div, /);
impl_op_array!(Rem, rem, %);
impl_op_array!(BitAnd, bitand, &);
impl_op_array!(BitOr, bitor, |);
impl_op_array!(BitXor, bitxor, ^);
impl_op_array!(Shl, shl, <<);
impl_op_array!(Shr, shr, >>);

// Implement the reference versions using StorageVec-specific implementations
impl_ref_op_vec!(Add, add, +);
impl_ref_op_vec!(Sub, sub, -);
// Replace multiplication with special implementation
impl_ref_mul_vec!();
// Replace division with special implementation
impl_ref_div_vec!();
impl_ref_op_vec!(Rem, rem, %);
impl_ref_op_vec!(BitAnd, bitand, &);
impl_ref_op_vec!(BitOr, bitor, |);
impl_ref_op_vec!(BitXor, bitxor, ^);
impl_ref_op_vec!(Shl, shl, <<);
impl_ref_op_vec!(Shr, shr, >>);

// Implement the reference versions for StorageArray
impl_ref_op_array!(Add, add, +);
impl_ref_op_array!(Sub, sub, -);
// Replace multiplication with special implementation
impl_ref_mul_array!();
// Replace division with special implementation
impl_ref_div_array!();
impl_ref_op_array!(Rem, rem, %);
impl_ref_op_array!(BitAnd, bitand, &);
impl_ref_op_array!(BitOr, bitor, |);
impl_ref_op_array!(BitXor, bitxor, ^);
impl_ref_op_array!(Shl, shl, <<);
impl_ref_op_array!(Shr, shr, >>);

// Implement the assignment operations for StorageVec
impl_op_assign_vec!(AddAssign, add_assign, +=);
impl_op_assign_vec!(SubAssign, sub_assign, -=);
impl_op_assign_vec!(MulAssign, mul_assign, *=);
impl_op_assign_vec!(DivAssign, div_assign, /=);
impl_op_assign_vec!(RemAssign, rem_assign, %=);
impl_op_assign_vec!(BitAndAssign, bitand_assign, &=);
impl_op_assign_vec!(BitOrAssign, bitor_assign, |=);
impl_op_assign_vec!(BitXorAssign, bitxor_assign, ^=);
impl_op_assign_vec!(ShlAssign, shl_assign, <<=);
impl_op_assign_vec!(ShrAssign, shr_assign, >>=);

// Implement the assignment operations for StorageArray
impl_op_assign_array!(AddAssign, add_assign, +=);
impl_op_assign_array!(SubAssign, sub_assign, -=);
impl_op_assign_array!(MulAssign, mul_assign, *=);
impl_op_assign_array!(DivAssign, div_assign, /=);
impl_op_assign_array!(RemAssign, rem_assign, %=);
impl_op_assign_array!(BitAndAssign, bitand_assign, &=);
impl_op_assign_array!(BitOrAssign, bitor_assign, |=);
impl_op_assign_array!(BitXorAssign, bitxor_assign, ^=);
impl_op_assign_array!(ShlAssign, shl_assign, <<=);
impl_op_assign_array!(ShrAssign, shr_assign, >>=);
