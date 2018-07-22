#![cfg(test)]

use std::panic::catch_unwind;
use super::matrix::*;

#[test]
fn test_add() {
	// Test square matrix
	{
		let inc = Matrix::new(3, 3, 0..9);
		let dec = Matrix::new(3, 3, (0..9).map(|n| 8-n));

		let res_copy = &inc + &dec;
		let res_move = inc + dec;
		assert_eq!(res_copy, res_move);

		for n in res_move {
			assert_eq!(n, 8);
		}
	}
	
	// Test != rows
	{
		let small = Matrix::new(3, 3, 0..9);
		let large = Matrix::new(6, 3, 0..18);

		let res = catch_unwind(|| small + large);
		assert!(res.is_err());
	}

	// Test != colums
	{
		let small = Matrix::new(3, 3, 0..9);
		let large = Matrix::new(3, 6, 0..18);

		let res = catch_unwind(|| small + large);
		assert!(res.is_err());
	}
}

#[test]
fn test_sub() {
	// Test square matrix
	{
		let inc = Matrix::new(3, 3, 0..9);
		let dec = Matrix::new(3, 3, 0..9);

		let res_copy = &inc - &dec;
		let res_move = inc - dec;
		assert_eq!(res_copy, res_move);

		for n in res_move {
			assert_eq!(n, 0);
		}
	}
	
	// Test != rows
	{
		let small = Matrix::new(3, 3, 0..9);
		let large = Matrix::new(6, 3, 0..18);

		let res = catch_unwind(|| small - large);
		assert!(res.is_err());
	}

	// Test != colums
	{
		let small = Matrix::new(3, 3, 0..9);
		let large = Matrix::new(3, 6, 0..18);

		let res = catch_unwind(|| small - large);
		assert!(res.is_err());
	}
}
