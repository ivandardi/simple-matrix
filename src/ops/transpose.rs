use crate::{MatrixArray, MatrixVec};

/// Trait for types that can be transposed
pub trait Transpose {
    /// The output type of the transpose operation.
    type Output;

    /// Transpose the object, returning a new object with rows and columns swapped
    fn transpose(&self) -> Self::Output;
}

impl<T> Transpose for MatrixVec<T>
where
    T: Default + Clone,
{
    type Output = MatrixVec<T>;

    fn transpose(&self) -> Self::Output {
        let mut transposed = MatrixVec::new(self.cols(), self.rows());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                transposed[[j, i]] = self[[i, j]].clone();
            }
        }
        transposed
    }
}

impl<T, const R: usize, const C: usize> Transpose for MatrixArray<T, R, C>
where
    T: Default + Copy,
{
    type Output = MatrixArray<T, C, R>;

    fn transpose(&self) -> Self::Output {
        let mut transposed = MatrixArray::<T, C, R>::new();
        for i in 0..R {
            for j in 0..C {
                transposed[[j, i]] = self[[i, j]].clone();
            }
        }
        transposed
    }
}
