use simple_matrix::{
    MatrixArray, MatrixVec,
    ops::{DotProduct, Transpose},
};

// ============================================================================
// MatrixVec Tests
// ============================================================================

#[test]
fn test_matrix_vec_new() {
    let matrix = MatrixVec::<i32>::new(3, 4);
    assert_eq!(matrix.rows(), 3);
    assert_eq!(matrix.cols(), 4);

    // Default value should be used for all elements
    for r in 0..3 {
        for c in 0..4 {
            assert_eq!(matrix[[r, c]], 0);
        }
    }
}

#[test]
fn test_matrix_vec_from_iter() {
    let matrix = MatrixVec::from_iter(2, 3, 0..6);

    // Check dimensions
    assert_eq!(matrix.rows(), 2);
    assert_eq!(matrix.cols(), 3);

    // Check values (row-major order)
    let expected = [[0, 1, 2], [3, 4, 5]];

    for r in 0..2 {
        for c in 0..3 {
            assert_eq!(matrix[[r, c]], expected[r][c]);
        }
    }
}

#[test]
#[should_panic(expected = "Matrix dimensions must be positive")]
fn test_matrix_vec_from_iter_zero_rows() {
    MatrixVec::<i32>::from_iter(0, 3, 0..0);
}

#[test]
#[should_panic(expected = "Matrix dimensions must be positive")]
fn test_matrix_vec_from_iter_zero_cols() {
    MatrixVec::<i32>::from_iter(3, 0, 0..0);
}

#[test]
#[should_panic(expected = "not enough elements in iterator")]
fn test_matrix_vec_from_iter_insufficient_elements() {
    // Create a 2x3 matrix but only provide 5 elements (we need 6)
    MatrixVec::<i32>::from_iter(2, 3, 0..5);
}

#[test]
fn test_matrix_vec_from_iter_excess_elements() {
    // Create a 2x2 matrix but provide more elements than needed
    // Extra elements should be ignored
    let matrix = MatrixVec::from_iter(2, 2, 0..10);

    assert_eq!(matrix.rows(), 2);
    assert_eq!(matrix.cols(), 2);

    let expected = [[0, 1], [2, 3]];

    for r in 0..2 {
        for c in 0..2 {
            assert_eq!(matrix[[r, c]], expected[r][c]);
        }
    }
}

#[test]
fn test_matrix_vec_identity() {
    let matrix = MatrixVec::<i32>::identity(3_usize, 1);

    // Check dimensions
    assert_eq!(matrix.rows(), 3);
    assert_eq!(matrix.cols(), 3);

    // Check that it's an identity matrix
    for r in 0..3 {
        for c in 0..3 {
            if r == c {
                assert_eq!(matrix[[r, c]], 1); // Should be 1 on the diagonal
            } else {
                assert_eq!(matrix[[r, c]], 0);
            }
        }
    }
}

#[test]
#[should_panic(expected = "Matrix size must be positive")]
fn test_matrix_vec_identity_zero_size() {
    MatrixVec::<i32>::identity(0_usize, 1);
}

#[test]
fn test_matrix_vec_get() {
    let matrix = MatrixVec::from_iter(2, 3, 0..6);

    // Valid indices
    assert_eq!(matrix.get(0_usize, 0_usize), Some(&0));
    assert_eq!(matrix.get(1_usize, 2_usize), Some(&5));

    // Out of bounds
    assert_eq!(matrix.get(2_usize, 0_usize), None);
    assert_eq!(matrix.get(0_usize, 3_usize), None);
    assert_eq!(matrix.get(2_usize, 3_usize), None);
}

#[test]
fn test_matrix_vec_get_mut() {
    let mut matrix = MatrixVec::from_iter(2, 3, 0..6);

    // Modify using get_mut
    if let Some(val) = matrix.get_mut(0_usize, 1_usize) {
        *val = 99;
    }

    assert_eq!(matrix[[0, 1]], 99);

    // Out of bounds
    assert_eq!(matrix.get_mut(2_usize, 0_usize), None);
}

#[test]
fn test_matrix_vec_set() {
    let mut matrix = MatrixVec::from_iter(2, 3, 0..6);

    // Valid set
    assert_eq!(matrix.set(0_usize, 2_usize, 99), Some(2));
    assert_eq!(matrix[[0, 2]], 99);

    // Out of bounds
    assert_eq!(matrix.set(2_usize, 0_usize, 99), None);
    assert_eq!(matrix.set(0_usize, 3_usize, 99), None);
}

#[test]
fn test_matrix_vec_get_row() {
    let matrix = MatrixVec::from_iter(3, 2, 0..6);

    // Valid row
    if let Some(row) = matrix.get_row(1_usize) {
        let row_vec: Vec<&i32> = row.into_iter().collect();
        assert_eq!(row_vec, vec![&2, &3]);
    } else {
        panic!("Expected Some, got None");
    }

    // Out of bounds
    assert!(matrix.get_row(3_usize).is_none());
}

#[test]
fn test_matrix_vec_get_col() {
    let matrix = MatrixVec::from_iter(3, 2, 0..6);

    // Valid column
    if let Some(col) = matrix.get_col(1_usize) {
        let col_vec: Vec<&i32> = col.into_iter().collect();
        assert_eq!(col_vec, vec![&1, &3, &5]);
    } else {
        panic!("Expected Some, got None");
    }

    // Out of bounds
    assert!(matrix.get_col(2_usize).is_none());
}

#[test]
fn test_matrix_vec_iter() {
    let matrix = MatrixVec::from_iter(2, 3, 0..6);
    let elements: Vec<&i32> = matrix.iter().collect();
    assert_eq!(elements, vec![&0, &1, &2, &3, &4, &5]);
}

#[test]
fn test_matrix_vec_iter_mut() {
    let mut matrix = MatrixVec::from_iter(2, 3, 0..6);

    // Double each element
    for val in matrix.iter_mut() {
        *val *= 2;
    }

    // Check the results
    let expected = [[0, 2, 4], [6, 8, 10]];

    for r in 0..2 {
        for c in 0..3 {
            assert_eq!(matrix[[r, c]], expected[r][c]);
        }
    }
}

#[test]
fn test_matrix_vec_index_array() {
    let matrix = MatrixVec::from_iter(2, 3, 0..6);

    assert_eq!(matrix[[0, 0]], 0);
    assert_eq!(matrix[[1, 2]], 5);
}

#[test]
#[should_panic(expected = "Invalid index")]
fn test_matrix_vec_index_array_out_of_bounds() {
    let matrix = MatrixVec::from_iter(2, 3, 0..6);
    let _ = matrix[[2, 0]];
}

#[test]
fn test_matrix_vec_index_tuple() {
    let matrix = MatrixVec::from_iter(2, 3, 0..6);

    assert_eq!(matrix[(0, 0)], 0);
    assert_eq!(matrix[(1, 2)], 5);
}

#[test]
#[should_panic(expected = "Invalid index")]
fn test_matrix_vec_index_tuple_out_of_bounds() {
    let matrix = MatrixVec::from_iter(2, 3, 0..6);
    let _ = matrix[(0, 3)];
}

#[test]
fn test_matrix_vec_index_mut_array() {
    let mut matrix = MatrixVec::from_iter(2, 3, 0..6);

    matrix[[0, 1]] = 99;
    assert_eq!(matrix[[0, 1]], 99);
}

#[test]
fn test_matrix_vec_index_mut_tuple() {
    let mut matrix = MatrixVec::from_iter(2, 3, 0..6);

    matrix[(1, 0)] = 99;
    assert_eq!(matrix[(1, 0)], 99);
}

// ============================================================================
// MatrixArray Tests
// ============================================================================

#[test]
fn test_matrix_array_new() {
    let matrix = MatrixArray::<i32, 2, 3>::new();

    assert_eq!(matrix.rows(), 2);
    assert_eq!(matrix.cols(), 3);

    // Default value should be used for all elements
    for r in 0..2 {
        for c in 0..3 {
            assert_eq!(matrix[[r, c]], 0);
        }
    }
}

#[test]
fn test_matrix_array_from_iter() {
    let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    // Check dimensions
    assert_eq!(matrix.rows(), 2);
    assert_eq!(matrix.cols(), 3);

    // Check values (row-major order)
    let expected = [[0, 1, 2], [3, 4, 5]];

    for r in 0..2 {
        for c in 0..3 {
            assert_eq!(matrix[[r, c]], expected[r][c]);
        }
    }
}

#[test]
#[should_panic(expected = "not enough elements in iterator")]
fn test_matrix_array_from_iter_insufficient_elements() {
    // Create a 2x3 matrix but only provide 5 elements (we need 6)
    // This should panic because we don't have enough elements
    MatrixArray::<i32, 2, 3>::from_iter(0..5);
}

#[test]
fn test_matrix_array_identity() {
    let matrix = MatrixArray::<i32, 3, 3>::identity(1);

    // Check dimensions
    assert_eq!(matrix.rows(), 3);
    assert_eq!(matrix.cols(), 3);

    // Check that it's an identity matrix
    for r in 0..3 {
        for c in 0..3 {
            if r == c {
                assert_eq!(matrix[[r, c]], 1); // Should be 1 on the diagonal
            } else {
                assert_eq!(matrix[[r, c]], 0);
            }
        }
    }
}

#[test]
#[should_panic(expected = "dimensions must match for an identity matrix")]
fn test_matrix_array_identity_non_square() {
    // This should panic since the array dimensions are not square (2x3)
    let _ = MatrixArray::<i32, 2, 3>::identity(1);
}

#[test]
fn test_matrix_array_get() {
    let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    // Valid indices
    assert_eq!(matrix.get(0_usize, 0_usize), Some(&0));
    assert_eq!(matrix.get(1_usize, 2_usize), Some(&5));

    // Out of bounds
    assert_eq!(matrix.get(2_usize, 0_usize), None);
    assert_eq!(matrix.get(0_usize, 3_usize), None);
    assert_eq!(matrix.get(2_usize, 3_usize), None);
}

#[test]
fn test_matrix_array_get_mut() {
    let mut matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    // Modify using get_mut
    if let Some(val) = matrix.get_mut(0_usize, 1_usize) {
        *val = 99;
    }

    assert_eq!(matrix[[0, 1]], 99);

    // Out of bounds
    assert_eq!(matrix.get_mut(2_usize, 0_usize), None);
}

#[test]
fn test_matrix_array_set() {
    let mut matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    // Valid set
    assert_eq!(matrix.set(0_usize, 2_usize, 99), Some(2));
    assert_eq!(matrix[[0, 2]], 99);

    // Out of bounds
    assert_eq!(matrix.set(2_usize, 0_usize, 99), None);
    assert_eq!(matrix.set(0_usize, 3_usize, 99), None);
}

#[test]
fn test_matrix_array_get_row() {
    let matrix = MatrixArray::<i32, 3, 2>::from_iter(0..6);

    // Valid row
    if let Some(row) = matrix.get_row(1_usize) {
        let row_vec: Vec<&i32> = row.into_iter().collect();
        assert_eq!(row_vec, vec![&2, &3]);
    } else {
        panic!("Expected Some, got None");
    }

    // Out of bounds
    assert!(matrix.get_row(3_usize).is_none());
}

#[test]
fn test_matrix_array_get_col() {
    let matrix = MatrixArray::<i32, 3, 2>::from_iter(0..6);

    // Valid column
    if let Some(col) = matrix.get_col(1_usize) {
        let col_vec: Vec<&i32> = col.into_iter().collect();
        assert_eq!(col_vec, vec![&1, &3, &5]);
    } else {
        panic!("Expected Some, got None");
    }

    // Out of bounds
    assert!(matrix.get_col(2_usize).is_none());
}

#[test]
fn test_matrix_array_iter() {
    let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);
    let elements: Vec<&i32> = matrix.iter().collect();
    assert_eq!(elements, vec![&0, &1, &2, &3, &4, &5]);
}

#[test]
fn test_matrix_array_iter_mut() {
    let mut matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    // Double each element
    for val in matrix.iter_mut() {
        *val *= 2;
    }

    // Check the results
    let expected = [[0, 2, 4], [6, 8, 10]];

    for r in 0..2 {
        for c in 0..3 {
            assert_eq!(matrix[[r, c]], expected[r][c]);
        }
    }
}

#[test]
fn test_matrix_array_index_array() {
    let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    assert_eq!(matrix[[0, 0]], 0);
    assert_eq!(matrix[[1, 2]], 5);
}

#[test]
#[should_panic(expected = "Invalid index")]
fn test_matrix_array_index_array_out_of_bounds() {
    let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);
    let _ = matrix[[2, 0]];
}

#[test]
fn test_matrix_array_index_tuple() {
    let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    assert_eq!(matrix[(0, 0)], 0);
    assert_eq!(matrix[(1, 2)], 5);
}

#[test]
#[should_panic(expected = "Invalid index")]
fn test_matrix_array_index_tuple_out_of_bounds() {
    let matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);
    let _ = matrix[(0, 3)];
}

#[test]
fn test_matrix_array_index_mut_array() {
    let mut matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    matrix[[0, 1]] = 99;
    assert_eq!(matrix[[0, 1]], 99);
}

#[test]
fn test_matrix_array_index_mut_tuple() {
    let mut matrix = MatrixArray::<i32, 2, 3>::from_iter(0..6);

    matrix[(1, 0)] = 99;
    assert_eq!(matrix[(1, 0)], 99);
}

// ============================================================================
// Edge Case and Boundary Tests
// ============================================================================

#[test]
fn test_matrix_vec_transpose_dimensions() {
    // Test that transposing correctly swaps dimensions
    let original = MatrixVec::<i32>::from_iter(2, 3, 0..6);
    assert_eq!(original.rows(), 2);
    assert_eq!(original.cols(), 3);

    let transposed = original.transpose();
    assert_eq!(transposed.rows(), 3);
    assert_eq!(transposed.cols(), 2);

    // Check the values
    for r in 0..original.rows() {
        for c in 0..original.cols() {
            assert_eq!(original[[r, c]], transposed[[c, r]]);
        }
    }
}

#[test]
fn test_matrix_array_transpose_dimensions() {
    // Test that transposing correctly swaps dimensions
    let original = MatrixArray::<i32, 2, 3>::from_iter(0..6);
    assert_eq!(original.rows(), 2);
    assert_eq!(original.cols(), 3);

    let transposed = original.transpose();
    assert_eq!(transposed.rows(), 3);
    assert_eq!(transposed.cols(), 2);

    // Check the values
    for r in 0..original.rows() {
        for c in 0..original.cols() {
            assert_eq!(original[[r, c]], transposed[[c, r]]);
        }
    }
}

#[test]
fn test_matrix_vec_extreme_values() {
    // Test with extreme values (min and max of i32)
    let values = vec![i32::MIN, i32::MAX, 0, 42];
    let matrix = MatrixVec::<i32>::from_iter(2, 2, values.clone());

    // Verify the extreme values are stored correctly
    assert_eq!(matrix[[0, 0]], i32::MIN);
    assert_eq!(matrix[[0, 1]], i32::MAX);
    assert_eq!(matrix[[1, 0]], 0);
    assert_eq!(matrix[[1, 1]], 42);

    // Check that iterator correctly yields these values
    let collected: Vec<&i32> = matrix.iter().collect();
    assert_eq!(collected, vec![&i32::MIN, &i32::MAX, &0, &42]);
}

#[test]
fn test_matrix_array_extreme_values() {
    // Test with extreme values (min and max of i32)
    let values = vec![i32::MIN, i32::MAX, 0, 42];
    let matrix = MatrixArray::<i32, 2, 2>::from_iter(values.clone());

    // Verify the extreme values are stored correctly
    assert_eq!(matrix[[0, 0]], i32::MIN);
    assert_eq!(matrix[[0, 1]], i32::MAX);
    assert_eq!(matrix[[1, 0]], 0);
    assert_eq!(matrix[[1, 1]], 42);

    // Check that iterator correctly yields these values
    let collected: Vec<&i32> = matrix.iter().collect();
    assert_eq!(collected, vec![&i32::MIN, &i32::MAX, &0, &42]);
}

#[test]
#[should_panic(expected = "dimensions must match")]
fn test_matrix_vec_incompatible_dimensions_add() {
    // Create matrices with different dimensions
    let matrix1 = MatrixVec::<i32>::from_iter(2, 3, 0..6);
    let matrix2 = MatrixVec::<i32>::from_iter(3, 2, 0..6);

    // This should panic because the dimensions don't match
    let _ = &matrix1 + &matrix2;
}

#[test]
#[should_panic(expected = "Matrix dimensions incompatible for dot product")]
fn test_matrix_vec_dot_product_incompatible_dimensions() {
    // Create matrices where matrix1 columns != matrix2 rows
    let matrix1 = MatrixVec::<i32>::from_iter(2, 3, 0..6);
    let matrix2 = MatrixVec::<i32>::from_iter(4, 2, 0..8);

    // This should panic because the dimensions aren't compatible for dot product
    let _ = matrix1.dot(&matrix2);
}

#[test]
fn test_matrix_vec_in_place_operations() {
    // Test in-place operations (+=, -=, *=, etc.)
    let mut matrix1 = MatrixVec::<i32>::from_iter(2, 2, 1..5);
    let matrix2 = MatrixVec::<i32>::from_iter(2, 2, 5..9);

    // Test +=
    matrix1 += &matrix2;
    assert_eq!(matrix1[[0, 0]], 6); // 1 + 5
    assert_eq!(matrix1[[0, 1]], 8); // 2 + 6
    assert_eq!(matrix1[[1, 0]], 10); // 3 + 7
    assert_eq!(matrix1[[1, 1]], 12); // 4 + 8

    // Test -=
    matrix1 -= &matrix2;
    assert_eq!(matrix1[[0, 0]], 1); // 6 - 5
    assert_eq!(matrix1[[0, 1]], 2); // 8 - 6
    assert_eq!(matrix1[[1, 0]], 3); // 10 - 7
    assert_eq!(matrix1[[1, 1]], 4); // 12 - 8

    // Test *=
    matrix1 *= &matrix2;
    assert_eq!(matrix1[[0, 0]], 5); // 1 * 5
    assert_eq!(matrix1[[0, 1]], 12); // 2 * 6
    assert_eq!(matrix1[[1, 0]], 21); // 3 * 7
    assert_eq!(matrix1[[1, 1]], 32); // 4 * 8

    // Test /=
    matrix1 /= &matrix2;
    assert_eq!(matrix1[[0, 0]], 1); // 5 / 5
    assert_eq!(matrix1[[0, 1]], 2); // 12 / 6
    assert_eq!(matrix1[[1, 0]], 3); // 21 / 7
    assert_eq!(matrix1[[1, 1]], 4); // 32 / 8
}

#[test]
fn test_matrix_array_in_place_operations() {
    // Test in-place operations (+=, -=, *=, etc.) with MatrixArray
    let mut matrix1 = MatrixArray::<i32, 2, 2>::from_iter(1..5);
    let matrix2 = MatrixArray::<i32, 2, 2>::from_iter(5..9);

    // Test +=
    matrix1 += &matrix2;
    assert_eq!(matrix1[[0, 0]], 6); // 1 + 5
    assert_eq!(matrix1[[0, 1]], 8); // 2 + 6
    assert_eq!(matrix1[[1, 0]], 10); // 3 + 7
    assert_eq!(matrix1[[1, 1]], 12); // 4 + 8

    // Test -=
    matrix1 -= &matrix2;
    assert_eq!(matrix1[[0, 0]], 1); // 6 - 5
    assert_eq!(matrix1[[0, 1]], 2); // 8 - 6
    assert_eq!(matrix1[[1, 0]], 3); // 10 - 7
    assert_eq!(matrix1[[1, 1]], 4); // 12 - 8

    // Test *=
    matrix1 *= &matrix2;
    assert_eq!(matrix1[[0, 0]], 5); // 1 * 5
    assert_eq!(matrix1[[0, 1]], 12); // 2 * 6
    assert_eq!(matrix1[[1, 0]], 21); // 3 * 7
    assert_eq!(matrix1[[1, 1]], 32); // 4 * 8

    // Test /=
    matrix1 /= &matrix2;
    assert_eq!(matrix1[[0, 0]], 1); // 5 / 5
    assert_eq!(matrix1[[0, 1]], 2); // 12 / 6
    assert_eq!(matrix1[[1, 0]], 3); // 21 / 7
    assert_eq!(matrix1[[1, 1]], 4); // 32 / 8
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn test_matrix_vec_division_by_zero() {
    // Test handling of division by zero
    let matrix1 = MatrixVec::<i32>::from_iter(2, 2, vec![10, 20, 30, 40]);
    let matrix2 = MatrixVec::<i32>::from_iter(2, 2, vec![2, 0, 3, 0]);

    // This will cause division by zero for elements at [0,1] and [1,1]
    // We expect a panic with "attempt to divide by zero"
    let _ = &matrix1 / &matrix2;
}

#[test]
fn test_matrix_vec_bitwise_operations() {
    // Test bitwise operations on matrices
    let matrix1 = MatrixVec::<i32>::from_iter(2, 2, vec![0b1010, 0b1100, 0b0011, 0b0101]);
    let matrix2 = MatrixVec::<i32>::from_iter(2, 2, vec![0b1111, 0b0011, 0b1010, 0b1100]);

    // Test bitwise AND
    let result_and = &matrix1 & &matrix2;
    assert_eq!(result_and[[0, 0]], 0b1010); // 0b1010 & 0b1111
    assert_eq!(result_and[[0, 1]], 0b0000); // 0b1100 & 0b0011
    assert_eq!(result_and[[1, 0]], 0b0010); // 0b0011 & 0b1010
    assert_eq!(result_and[[1, 1]], 0b0100); // 0b0101 & 0b1100

    // Test bitwise OR
    let result_or = &matrix1 | &matrix2;
    assert_eq!(result_or[[0, 0]], 0b1111); // 0b1010 | 0b1111
    assert_eq!(result_or[[0, 1]], 0b1111); // 0b1100 | 0b0011
    assert_eq!(result_or[[1, 0]], 0b1011); // 0b0011 | 0b1010
    assert_eq!(result_or[[1, 1]], 0b1101); // 0b0101 | 0b1100

    // Test bitwise XOR
    let result_xor = &matrix1 ^ &matrix2;
    assert_eq!(result_xor[[0, 0]], 0b0101); // 0b1010 ^ 0b1111
    assert_eq!(result_xor[[0, 1]], 0b1111); // 0b1100 ^ 0b0011
    assert_eq!(result_xor[[1, 0]], 0b1001); // 0b0011 ^ 0b1010
    assert_eq!(result_xor[[1, 1]], 0b1001); // 0b0101 ^ 0b1100
}

#[test]
fn test_matrix_array_transpose_1x_n_nx1() {
    // Test transposing a 1×N matrix (row vector) to an N×1 matrix (column vector)
    let row_vector = MatrixArray::<i32, 1, 3>::from_iter(vec![1, 2, 3]);
    assert_eq!(row_vector.rows(), 1);
    assert_eq!(row_vector.cols(), 3);

    // Transpose to column vector
    let col_vector = row_vector.transpose();
    assert_eq!(col_vector.rows(), 3);
    assert_eq!(col_vector.cols(), 1);

    // Check values
    assert_eq!(col_vector[[0, 0]], 1);
    assert_eq!(col_vector[[1, 0]], 2);
    assert_eq!(col_vector[[2, 0]], 3);

    // Now transpose the column vector back to a row vector
    let back_to_row = col_vector.transpose();
    assert_eq!(back_to_row.rows(), 1);
    assert_eq!(back_to_row.cols(), 3);

    // Check values are preserved through double transpose
    assert_eq!(back_to_row[[0, 0]], 1);
    assert_eq!(back_to_row[[0, 1]], 2);
    assert_eq!(back_to_row[[0, 2]], 3);
}

#[test]
fn test_matrix_vec_transpose_1x_n_nx1() {
    // Test transposing a 1×N matrix (row vector) to an N×1 matrix (column vector)
    let row_vector = MatrixVec::<i32>::from_iter(1, 3, vec![1, 2, 3]);
    assert_eq!(row_vector.rows(), 1);
    assert_eq!(row_vector.cols(), 3);

    // Transpose to column vector
    let col_vector = row_vector.transpose();
    assert_eq!(col_vector.rows(), 3);
    assert_eq!(col_vector.cols(), 1);

    // Check values
    assert_eq!(col_vector[[0, 0]], 1);
    assert_eq!(col_vector[[1, 0]], 2);
    assert_eq!(col_vector[[2, 0]], 3);
}

#[test]
fn test_matrix_vec_dot_product_single_element() {
    // Test dot product of matrices that produce a single element result
    // (1x3) * (3x1) = (1x1)
    let row_vector = MatrixVec::<i32>::from_iter(1, 3, vec![1, 2, 3]);
    let col_vector = MatrixVec::<i32>::from_iter(3, 1, vec![4, 5, 6]);

    // Dot product should be a 1×1 matrix
    let result = row_vector.dot(&col_vector);
    assert_eq!(result.rows(), 1);
    assert_eq!(result.cols(), 1);

    // Value should be the sum of products: 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    assert_eq!(result[[0, 0]], 32);
}

#[test]
fn test_matrix_vec_identity_with_dot_product() {
    // Test that multiplying a matrix by an identity matrix returns the original matrix
    let matrix = MatrixVec::<i32>::from_iter(2, 3, 1..7); // 1,2,3,4,5,6
    let identity = MatrixVec::<i32>::identity(3usize, 1);

    // Check matrix * identity
    let result = matrix.dot(&identity);
    assert_eq!(result.rows(), matrix.rows());
    assert_eq!(result.cols(), matrix.cols());

    // Result should equal original matrix
    for r in 0..matrix.rows() {
        for c in 0..matrix.cols() {
            assert_eq!(result[[r, c]], matrix[[r, c]]);
        }
    }
}
