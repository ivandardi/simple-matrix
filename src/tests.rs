#![cfg(test)]

use std::panic::catch_unwind;
use super::matrix::*;

#[test]
fn test_add() {
	// Test square matrix
	{
		let inc = Matrix::new(3, 3, 0..9);

		let dec = Matrix::new(3, 3, (0..9).map(|n| 8-n));

		let res = &inc + &dec;
		for i in 0..9 {
			assert_eq!(res.data[i], 8);
		}
	}
	
	// Test != rows
	{
		let mut small = Matrix::new(3, 3, 0..9);
		for i in 0..9 {
			small.data[i] = i;
		}

		let mut large = Matrix::new(6, 3, 0..18);
		for i in 0..9 {
			large.data[i] = 8-i;
		}

		let res = catch_unwind(|| &small + &large);
		assert!(res.is_err());
	}

	// Test != colums
	{
		let mut small = Matrix::new(3, 3, 0..9);
		for i in 0..9 {
			small.data[i] = i;
		}

		let mut large = Matrix::new(3, 6, 0..18);
		for i in 0..9 {
			large.data[i] = 8-i;
		}

		let res = catch_unwind(|| &small + &large);
		assert!(res.is_err());
	}
}
