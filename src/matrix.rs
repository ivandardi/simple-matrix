use std::ops::Sub;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
	pub data: Vec<T>,
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
			rows: rows,
			cols: cols
		}
	}

	pub fn get<'a>(&'a self, row: usize, col: usize) -> Option<&'a T> {
		if row < self.rows && col < self.cols {
			Some(&self.data[row + col * self.rows])
		} else {
			None
		}
	}

	pub fn get_mut<'a>(&'a mut self, row: usize, col: usize) -> Option<&'a mut T> {
		if row < self.rows && col < self.cols {
			Some(&mut self.data[row + col * self.rows])
		} else {
			None
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
