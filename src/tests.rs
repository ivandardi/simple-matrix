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
fn test_get() {
    let inc = Matrix::new(3, 5, 0..);

    // Test correct cells
    for row in 0..3 {
        for col in 0..5 {
            let val = inc.get(row, col);
            assert!(val.is_some());
            assert_eq!(*val.unwrap(), col + row * inc.cols);
        }
    }

    // Test incorrect rows
    for row in 3..6 {
        for col in 0..5 {
            let val = inc.get(row, col);
            assert!(val.is_none());
        }
    }

    // Test incorrect cols
    for row in 0..3 {
        for col in 5..10 {
            let val = inc.get(row, col);
            assert!(val.is_none());
        }
    }

    // Test incorrect rows & cols
    for row in 3..6 {
        for col in 5..10 {
            let val = inc.get(row, col);
            assert!(val.is_none());
        }
    }
}

#[test]
fn test_transpose() {
    // Test square matrix
    {
        let inc = Matrix::new(3, 3, 0..);
        let double = inc.transpose().transpose();
        assert_eq!(inc, double);
    }

    // Test rectangular matrix
    {
        let inc = Matrix::new(2, 5, 0..);
        let double = inc.transpose().transpose();
        assert_eq!(inc, double);
    }

    // Test 0-row matrix
    {
        let inc = Matrix::new(0, 3, 0..);
        let double = inc.transpose().transpose();
        assert_eq!(inc, double);
    }

    // Test 0-col matrix
    {
        let inc = Matrix::new(3, 0, 0..);
        let double = inc.transpose().transpose();
        assert_eq!(inc, double);
    }
}

#[test]
fn test_add() {
    macro_rules! test_matrix {
        ($rows:expr, $cols:expr) => {{
            let len = $rows * $cols;

            // inc matrix
            let inc = Matrix::new($rows, $cols, 0..);

            let res_copy = &inc + &inc;
            let res_move = inc.clone() + inc;
            assert_eq!(res_copy, res_move);

            for (i, n) in res_move.into_iter().enumerate() {
                assert_eq!(n, 2 * i);
            }

            // dec matrix
            let dec = Matrix::new($rows, $cols, (0..len).rev());

            let res_copy = &dec + &dec;
            let res_move = dec.clone() + dec;
            assert_eq!(res_copy, res_move);

            for (i, n) in res_move.into_iter().enumerate() {
                assert_eq!(n, 2 * (len - 1 - i));
            }
        }};
    }

    // Test square matrix
    test_matrix!(3, 3);

    // Test rectangle matrix
    test_matrix!(2, 5);

    // Test 0-row matrix
    test_matrix!(0, 3);

    // Test 0-col matrix
    test_matrix!(3, 0);

    // Test != rows & != cols
    assert_dif_rows_panic!(|a, b| a + b);
    assert_dif_cols_panic!(|a, b| a + b);
}

#[test]
fn test_sub() {
    macro_rules! test_matrix {
        ($rows:expr, $cols:expr) => {{
            // inc matrix
            let inc = Matrix::new($rows, $cols, 0..);

            let res_copy = &inc - &inc;
            let res_move = inc.clone() - inc;
            assert_eq!(res_copy, res_move);

            for n in res_move {
                assert_eq!(n, 0);
            }

            // dec matrix
            let dec = Matrix::new($rows, $cols, (0..$rows * $cols).rev());

            let res_copy = &dec - &dec;
            let res_move = dec.clone() - dec;
            assert_eq!(res_copy, res_move);

            for n in res_move {
                assert_eq!(n, 0);
            }
        }};
    }
    // Test square matrix
    test_matrix!(3, 3);

    // Test rectangle matrix
    test_matrix!(2, 5);

    // Test 0-row matrix
    test_matrix!(0, 3);

    // Test 0-col matrix
    test_matrix!(3, 0);

    // Test != rows & != cols
    assert_dif_rows_panic!(|a, b| a - b);
    assert_dif_cols_panic!(|a, b| a - b);
}

#[test]
fn test_mul() {
    // Test square matrix (1)
    {
        let inc = Matrix::new(3, 3, 0..);
        let inc2 = Matrix::new(3, 3, 0..);
        let res = inc * inc2;
        let correct = Matrix::new(3, 3, vec![15, 18, 21, 42, 54, 66, 69, 90, 111]);
        assert_eq!(res, correct);
    }

    // Test square matrix (2)
    {
        let inc = Matrix::new(3, 3, 0..);
        let dec = Matrix::new(3, 3, (0..9).map(|n| 8 - n));
        let res = inc * dec;
        let correct = Matrix::new(3, 3, vec![9, 6, 3, 54, 42, 30, 99, 78, 57]);
        assert_eq!(res, correct);
    }

    // Test transposed matrix (1)
    {
        let inc = Matrix::new(2, 3, 0..);
        let inc_t = inc.transpose();
        let res = inc * inc_t;
        let correct = Matrix::new(2, 2, vec![10, 13, 28, 40]);
        assert_eq!(res, correct);
    }

    // Test transposed matrix (2)
    {
        let dec = Matrix::new(3, 2, (0..6).map(|n| 5 - n));
        let dec_t = dec.transpose();
        let res = dec * dec_t;
        let correct = Matrix::new(3, 3, vec![33, 24, 15, 19, 14, 9, 5, 4, 3]);
        assert_eq!(res, correct);
    }
}
