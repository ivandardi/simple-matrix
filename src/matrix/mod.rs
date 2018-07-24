mod iter;
mod std_ops;

#[derive(Debug, Clone, Default)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new<I>(rows: usize, cols: usize, data: I) -> Matrix<T>
    where
        I: IntoIterator<Item = T>,
    {
        Matrix {
            rows,
            cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[col + row * self.cols])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[col + row * self.cols])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
            true
        } else {
            false
        }
    }

    pub fn row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows {
            Some((0..self.cols).map(move |col| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.cols {
            Some((0..self.rows).map(move |row| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn transpose(&self) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);
                for row in 0..self.rows {
                    for val in self.row(row).unwrap() {
                        data.push(val.clone());
                    }
                }
                data
            },
        }
    }

    pub fn apply<F: Fn(&mut T)>(&mut self, func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}
