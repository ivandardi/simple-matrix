// An example demonstrating vector matrix operations
use simple_matrix::{
    Matrix, MatrixVec,
    ops::{DotProduct, Transpose},
};

fn main() {
    println!("Basic Vector Matrix Example");
    println!("==========================");

    // Create a 2x3 matrix with default values (zeros)
    let zeros: MatrixVec<i32> = MatrixVec::<i32>::new(2, 3);
    println!("Zeros matrix (2x3):");
    print_matrix(&zeros);

    // Create a 2x3 matrix from a vector of values
    let mat1 = MatrixVec::<i32>::from_iter(2, 3, vec![1, 2, 3, 4, 5, 6]);
    println!("\nMatrix 1 (2x3):");
    print_matrix(&mat1);

    // Create another matrix with different values
    let mat2 = MatrixVec::<i32>::from_iter(2, 3, vec![7, 8, 9, 10, 11, 12]);
    println!("\nMatrix 2 (2x3):");
    print_matrix(&mat2);

    // Matrix addition
    let sum: MatrixVec<i32> = &mat1 + &mat2;
    println!("\nMatrix 1 + Matrix 2:");
    print_matrix(&sum);

    // Matrix subtraction
    let diff: MatrixVec<i32> = &mat1 - &mat2;
    println!("\nMatrix 1 - Matrix 2:");
    print_matrix(&diff);

    // Element-wise multiplication
    let elem_mul: MatrixVec<i32> = &mat1 * &mat2;
    println!("\nMatrix 1 * Matrix 2 (element-wise):");
    print_matrix(&elem_mul);

    // Transpose a matrix
    let transposed: MatrixVec<i32> = mat1.transpose();
    println!("\nMatrix 1 Transposed:");
    print_matrix(&transposed);

    // Matrix multiplication (dot product)
    let dot_product: MatrixVec<i32> = mat1.dot(&transposed);
    println!("\nMatrix 1 · Matrix 1ᵀ (dot product):");
    print_matrix(&dot_product);

    // Accessing elements
    println!("\nAccessing elements of Matrix 1:");
    println!("mat1[0,0] = {}", mat1[[0, 0]]);
    println!("mat1[0,1] = {}", mat1[(0, 1)]);
    println!("mat1[1,2] = {}", mat1.get(1usize, 2usize).unwrap());

    // Using iterators
    let mut doubled = mat1.clone();
    doubled.iter_mut().for_each(|x| *x *= 2);
    println!("\nMatrix 1 with all elements doubled:");
    print_matrix(&doubled);
}

// Helper function to print a matrix
fn print_matrix<T: std::fmt::Display + Copy, S>(matrix: &Matrix<T, S>)
where
    S: simple_matrix::storage::Storage<T>,
{
    for r in 0..matrix.rows() {
        print!("[ ");
        for c in 0..matrix.cols() {
            print!("{:4} ", matrix[[r, c]]);
        }
        println!("]");
    }
}
