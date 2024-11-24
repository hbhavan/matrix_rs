use crate::matrix::Matrix;
use rand::Rng;
use std::time::Instant;

#[allow(dead_code)]
fn matrix_test_1() {
    let mut mat_a = Matrix::<f32>::new(2, 2);
    let mat_b = Matrix::<f32>::new(2, 5);
    let mat_c = Matrix::<f32>::new(2, 5);

    mat_a
        .set(0, 0, 413.0)
        .and_then(|m| m.set(0, 1, 55.0))
        .and_then(|m| m.set(1, 0, 2.0))
        .and_then(|m| m.set(1, 1, 27492.0));

    let mat_d = mat_b.add(3.0);
    let mat_e = mat_c.add(4.0);
}

#[allow(dead_code)]
fn matrix_test_2() {
    let mut mat_a: Matrix<f32> = Matrix::new(2, 3);
    let mut mat_b: Matrix<f32> = Matrix::new(3, 2);

    mat_a.set(0, 0, 1.0);
    mat_a.set(0, 1, 2.0);
    mat_a.set(0, 2, 3.0);
    mat_a.set(1, 0, 4.0);
    mat_a.set(1, 1, 5.0);
    mat_a.set(1, 2, 6.0);

    mat_b.set(0, 0, 7.0);
    mat_b.set(0, 1, 8.0);
    mat_b.set(1, 0, 9.0);
    mat_b.set(1, 1, 10.0);
    mat_b.set(2, 0, 11.0);
    mat_b.set(2, 1, 12.0);

    println!("{}", mat_a.to_string());
    println!("{}", mat_b.to_string());
    let mat_c = mat_a.matrix_multiply(&mat_b).expect("Invalid rows");

    println!("{}", mat_c.to_string());
}

#[allow(dead_code)]
fn matrix_mult_test_1() {
    let mat_a = Matrix::<f32>::from_vec(vec![vec![1.0, 2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0, 8.0]]);
    let mat_b = Matrix::<f32>::from_vec(vec![
        vec![1.0, 2.0],
        vec![1.0, 2.0],
        vec![1.0, 2.0],
        vec![1.0, 2.0],
    ]);

    let mat_c = mat_a.matrix_multiply(&mat_b);
    Matrix::print_matrix(mat_c);
}

#[allow(dead_code)]
fn matrix_mult_test_2() {
    let mat_a = Matrix::from_vec(vec![vec![9.0, 2.0, 12.0, 4.0], vec![2.0, 8.0, 21.0, 55.0]]);
    let mat_b = Matrix::from_vec(vec![vec![7.0], vec![2.0], vec![92.0], vec![3.0]]);
    let mat_c = mat_a.matrix_multiply2(&mat_b);

    Matrix::print_matrix(mat_c);

    let mat_a = Matrix::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let mat_b = Matrix::from_vec(vec![vec![4.0], vec![5.0], vec![6.0]]);
    let mat_c = mat_a.matrix_multiply2(&mat_b);

    Matrix::print_matrix(mat_c);
}

#[allow(dead_code)]
fn matrix_mult_test_3() {
    let mat_a = Matrix::from_vec(vec![
        vec![3.0, 3.0, 3.0],
        vec![4.0, 4.0, 4.0],
        vec![5.0, 5.0, 5.0],
    ]);
    let mat_b = Matrix::from_vec(vec![
        vec![4.0, 7.0, 5.0],
        vec![5.0, 8.0, 6.0],
        vec![6.0, 9.0, 7.0],
    ]);
    let mat_c = mat_a.matrix_multiply(&mat_b);

    Matrix::print_matrix(mat_c);

    let mat_a = Matrix::from_vec(vec![
        vec![3.0, 3.0, 3.0],
        vec![4.0, 4.0, 4.0],
        vec![5.0, 5.0, 5.0],
    ]);
    let mat_b = Matrix::from_vec(vec![vec![4.0, 7.0], vec![5.0, 8.0], vec![6.0, 9.0]]);
    let mat_c = mat_a.matrix_multiply(&mat_b);

    Matrix::print_matrix(mat_c);
}

#[allow(dead_code)]
fn matrix_equality_test_1() {
    let mut rng = rand::thread_rng();
    let a: f32 = rng.gen::<f32>() * 100.0;
    let b: f32 = rng.gen::<f32>() * 100.0;
    let c: f32 = rng.gen::<f32>() * 100.0;

    let mat_a = Matrix::from_vec(vec![vec![a, b, c], vec![b, c, a], vec![c, a, b]]);
    let mat_b = Matrix::from_vec(vec![vec![c, b, a], vec![a, c, b], vec![b, a, c]]);

    let mat_c = mat_a.matrix_multiply(&mat_b).unwrap();
    let mat_d = mat_a.matrix_multiply2(&mat_b).unwrap();

    if !mat_c.equals(&mat_d) {
        println!("MISMATCH FOUND: ");
        println!("Matrix A: {}", mat_a.to_string());
        println!("Matrix B: {}", mat_b.to_string());
        println!("================================");

        println!("Matrix mutliply 1: {}", mat_c.to_string());
        println!("Matrix mutliply 2: {}", mat_d.to_string());
    } else {
        println!("Matrix multiplication successful");
    }
}

#[allow(dead_code)]
fn matrix_equality_test_2() {
    let mut rng = rand::thread_rng();

    let mat_a_rows = rng.gen_range(1..5);
    let mat_size = rng.gen_range(1..5);
    let mat_b_cols = rng.gen_range(1..5);

    let mut mat_a = Matrix::new(mat_a_rows, mat_size);
    let mut mat_b = Matrix::new(mat_size, mat_b_cols);

    for i in 0..mat_a_rows * mat_size {
        let rand = rng.gen::<f32>() * 100.0;
        let (x, y) = Matrix::get_coords(i, mat_a_rows, mat_size).unwrap_or((0, 0));
        mat_a.set(x, y, rand);
    }

    for i in 0..mat_size * mat_b_cols {
        let rand = rng.gen::<f32>() * 100.0;
        let (x, y) = Matrix::get_coords(i, mat_size, mat_b_cols).unwrap_or((0, 0));
        mat_b.set(x, y, rand);
    }

    let mat_c = mat_a.matrix_multiply(&mat_b).unwrap();
    let mat_d = mat_a.matrix_multiply2(&mat_b).unwrap();

    if !mat_c.equals(&mat_d) {
        println!("MISMATCH FOUND: ");
        println!("Matrix A: {}", mat_a.to_string());
        println!("Matrix B: {}", mat_b.to_string());
        println!("================================");

        println!("Matrix mutliply 1: {}", mat_c.to_string());
        println!("Matrix mutliply 2: {}", mat_d.to_string());
    } else {
        println!("Matrix multiplication successful");
    }
}

fn matrix_mult_1_rand_test() {
    let mut rng = rand::thread_rng();

    let mat_a_rows = rng.gen_range(1..5);
    let mat_size = rng.gen_range(1..5);
    let mat_b_cols = rng.gen_range(1..5);

    let mut mat_a = Matrix::new(mat_a_rows, mat_size);
    let mut mat_b = Matrix::new(mat_size, mat_b_cols);

    for i in 0..mat_a_rows * mat_size {
        let rand = rng.gen::<f32>() * 100.0;
        let (x, y) = Matrix::get_coords(i, mat_a_rows, mat_size).unwrap_or((0, 0));
        mat_a.set(x, y, rand);
    }

    for i in 0..mat_size * mat_b_cols {
        let rand = rng.gen::<f32>() * 100.0;
        let (x, y) = Matrix::get_coords(i, mat_size, mat_b_cols).unwrap_or((0, 0));
        mat_b.set(x, y, rand);
    }

    let _mat_c = mat_a.matrix_multiply(&mat_b).unwrap();
}

fn matrix_mult_2_rand_test() {
    let mut rng = rand::thread_rng();

    let mat_a_rows = rng.gen_range(1..5);
    let mat_size = rng.gen_range(1..5);
    let mat_b_cols = rng.gen_range(1..5);

    let mut mat_a = Matrix::new(mat_a_rows, mat_size);
    let mut mat_b = Matrix::new(mat_size, mat_b_cols);

    for i in 0..mat_a_rows * mat_size {
        let rand = rng.gen::<f32>() * 100.0;
        let (x, y) = Matrix::get_coords(i, mat_a_rows, mat_size).unwrap_or((0, 0));
        mat_a.set(x, y, rand);
    }

    for i in 0..mat_size * mat_b_cols {
        let rand = rng.gen::<f32>() * 100.0;
        let (x, y) = Matrix::get_coords(i, mat_size, mat_b_cols).unwrap_or((0, 0));
        mat_b.set(x, y, rand);
    }

    let _mat_d = mat_a.matrix_multiply2(&mat_b).unwrap();
}

fn matrix_mutliply_speed_test() {
    let lim = 250000;

    let mat_mult_1_start = Instant::now();
    for _num in 0..lim {
        matrix_mult_1_rand_test();
    }
    let mat_mult_1_end = mat_mult_1_start.elapsed();

    println!("Matrix Multiply 1 Duration: {:?}", mat_mult_1_end);

    let mat_mult_2_start = Instant::now();
    for _num in 0..lim {
        matrix_mult_2_rand_test();
    }
    let mat_mult_2_end = mat_mult_2_start.elapsed();

    println!("Matrix Multiply 2 Duration: {:?}", mat_mult_2_end);
}
