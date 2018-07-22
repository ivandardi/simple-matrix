use std::ops::Sub;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
	data: Vec<T>,
	pub rows: usize,
	pub cols: usize
}

impl<T> Matrix<T> {
	pub fn new<U>(rows: usize, cols: usize, data: U) -> Matrix<T>
		where T: Copy + Default,
			U: IntoIterator<Item=T> {
		Matrix {
			data: {
				let data: Vec<_> = data.into_iter().take(rows*cols).collect();
				assert_eq!(data.len(), rows * cols);
				data
			},
			rows,
			cols
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

	pub fn transpose(&self) -> Matrix<T> where T: Copy {
		Matrix {
			data: {
				let mut data = Vec::with_capacity(self.cols*self.rows);
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

	pub fn apply<F>(&mut self, func: F) where F: Fn(&mut T) {
		self.data.iter_mut().for_each(|n| func(n));
	}
}

// PartialEq implementation

impl<T> PartialEq for Matrix<T> where T: PartialEq + ::std::fmt::Display {
	fn eq(&self, rhs: &Self) -> bool {
		self.rows == rhs.rows &&
		self.cols == rhs.cols &&
		self.data.iter().zip(rhs.data.iter())
			.all(|(a, b)| *a == *b)
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

impl<T> Add for Matrix<T> where T: Add<Output=T> + Copy {
	type Output = Matrix<T>;

	fn add(mut self, other: Matrix<T>) -> Matrix<T> {
		// Reuse self because memory for result

		assert!(self.rows == other.rows);
		assert!(self.cols == other.cols);

		for (a, b) in (&mut self).into_iter().zip(other.into_iter()) {
			*a = *a + b;
		}

		self
	}
}

impl<'a, T> Add for &'a Matrix<T> where T: Add<Output=T> + Copy {
	type Output = Matrix<T>;

	fn add<'b>(self, other: &'b Matrix<T>) -> Matrix<T> {
		assert!(self.rows == other.rows);
		assert!(self.cols == other.cols);

		Matrix {
			data: self.data.iter()
				.zip(other.data.iter())
				.map(|(a, b)| *a + *b)
				.collect(),
			rows: self.rows,
			cols: self.cols
		}
	}
}

// Sub implementation

impl<T> Sub for Matrix<T> where T: Sub<Output=T> + Copy {
	type Output = Matrix<T>;

	fn sub(mut self, other: Matrix<T>) -> Matrix<T> {
		// Reuse self because memory for result

		assert!(self.rows == other.rows);
		assert!(self.cols == other.cols);

		for (a, b) in (&mut self).into_iter().zip(other.into_iter()) {
			*a = *a - b;
		}

		self
	}
}

impl<'a, T> Sub for &'a Matrix<T> where T: Sub<Output=T> + Copy {
	type Output = Matrix<T>;

	fn sub<'b>(self, other: &'b Matrix<T>) -> Matrix<T> {
		assert!(self.rows == other.rows);
		assert!(self.cols == other.cols);

		Matrix {
			data: self.data.iter()
				.zip(other.data.iter())
				.map(|(a, b)| *a - *b)
				.collect(),
			rows: self.rows,
			cols: self.cols
		}
	}
}
