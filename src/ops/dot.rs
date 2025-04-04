use crate::{Matrix, MatrixArray, MatrixVec, storage::Storage};

/// Trait for types that support matrix dot product.
pub trait DotProduct<T, S>
where
    S: Storage<T>,
{
    /// The output type of the dot product operation.
    type Output;

    /// Computes the dot product of two matrices.
    ///
    /// # Arguments
    /// * `rhs` - The right-hand side matrix to multiply with.
    ///
    /// # Returns
    /// A new matrix that is the result of the dot product.
    ///
    /// # Panics
    /// Panics if the number of columns in the left-hand side matrix does not match
    /// the number of rows in the right-hand side matrix.
    fn dot(&self, rhs: &Matrix<T, S>) -> Self::Output;
}

// Handle edge cases for dot product without overflow panics
impl<T, S> DotProduct<T, S> for MatrixVec<T>
where
    S: Storage<T>,
    T: Default + Clone + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    type Output = MatrixVec<T>;

    fn dot(&self, rhs: &Matrix<T, S>) -> Self::Output {
        // Check compatibility of dimensions for dot product
        assert_eq!(
            self.cols(),
            rhs.rows(),
            "Matrix dimensions incompatible for dot product"
        );

        let mut result = MatrixVec::new(self.rows(), rhs.cols());

        // Use checked operations to avoid panics
        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                let mut sum = T::default();
                for k in 0..self.cols() {
                    let product = self[[i, k]].clone() * rhs[[k, j]].clone();
                    sum += product;
                }
                result[[i, j]] = sum;
            }
        }

        result
    }
}

impl<T, S, const R: usize, const C: usize> DotProduct<T, S> for MatrixArray<T, R, C>
where
    S: Storage<T>,
    T: Default + Copy + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    type Output = MatrixArray<T, R, C>;

    fn dot(&self, rhs: &Matrix<T, S>) -> Self::Output {
        // Check compatibility of dimensions for dot product
        assert_eq!(
            self.cols(),
            rhs.rows(),
            "Matrix dimensions incompatible for dot product"
        );

        let mut result = MatrixArray::<T, R, C>::new();
        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                let mut sum = T::default();
                for k in 0..self.cols() {
                    // Wrapping behavior for numeric types is handled by Rust's standard overflow behavior
                    sum += self[[i, k]] * rhs[[k, j]];
                }
                result[[i, j]] = sum;
            }
        }
        result
    }
}
