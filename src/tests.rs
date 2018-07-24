#![cfg(test)]

use super::matrix::*;

macro_rules! assert_panic {
    ($x:expr) => {{
        let res = ::std::panic::catch_unwind(|| $x);
        assert!(res.is_err());
    }};
}

macro_rules! assert_dif_rows_panic {
    ($func:expr) => {{
        let small = Matrix::new(3, 3, 0..);
        let large = Matrix::new(6, 3, 0..);

        // Reference tests
        assert_panic!($func(&small, &large));
        assert_panic!($func(&large, &small));

        // Value tests
        assert_panic!($func(small.clone(), large.clone()));
        assert_panic!($func(large, small));
    }};
}

macro_rules! assert_dif_cols_panic {
    ($func:expr) => {{
        let small = Matrix::new(3, 3, 0..);
        let large = Matrix::new(3, 6, 0..);

        // Reference tests
        assert_panic!($func(&small, &large));
        assert_panic!($func(&large, &small));

        // Value tests
        assert_panic!($func(small.clone(), large.clone()));
        assert_panic!($func(large, small));
    }};
}

#[test]
fn test_add() {
    // Test square matrix
    {
        let inc = Matrix::new(3, 3, 0..);
        let dec = Matrix::new(3, 3, (0..9).map(|n| 8 - n));

        let res_copy = &inc + &dec;
        let res_move = inc + dec;
        assert_eq!(res_copy, res_move);

        for n in res_move {
            assert_eq!(n, 8);
        }
    }

    // Test != rows & != cols
    assert_dif_rows_panic!(|a, b| a + b);
    assert_dif_cols_panic!(|a, b| a + b);
}

#[test]
fn test_sub() {
    // Test square matrix
    {
        let inc = Matrix::new(3, 3, 0..);
        let dec = Matrix::new(3, 3, 0..);

        let res_copy = &inc - &dec;
        let res_move = inc - dec;
        assert_eq!(res_copy, res_move);

        for n in res_move {
            assert_eq!(n, 0);
        }
    }

    // Test != rows & != cols
    assert_dif_rows_panic!(|a, b| a - b);
    assert_dif_cols_panic!(|a, b| a - b);
}
