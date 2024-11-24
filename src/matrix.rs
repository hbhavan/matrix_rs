use std::fmt;
use std::fmt::Display;
use std::iter::zip;
use std::ops::{Add, Div, Mul, Sub};
use std::slice::Chunks;

#[derive(Debug)]
pub struct Matrix<T>
where
    T: Default,
{
    rows: usize,
    cols: usize,
    matrix: Vec<T>,
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Default + Copy + Clone,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: Vec::new(),
        }
    }

    pub fn new_empty(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: vec![Default::default(); rows * cols],
        }
    }

    pub fn from_vec(v: Vec<Vec<T>>) -> Self {
        Self {
            rows: v.len(),
            cols: v[0].len(),
            matrix: v.into_iter().flatten().collect(),
        }
    }

    pub fn num_rows(&self) -> usize {
        return self.rows;
    }

    pub fn num_cols(&self) -> usize {
        return self.cols;
    }

    pub fn index_inbounds(&self, row: usize, col: usize) -> Option<usize> {
        return match (self.rows, self.cols, row, col) {
            (rows, _, x, _) if rows < x => None,
            (_, cols, _, y) if cols < y => None,
            (_, cols, x, y) => Some(x * cols + y),
        };
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        return row * self.cols + col;
    }

    pub fn at(&self, row: usize, col: usize) -> Option<&T> {
        return self.matrix.get(self.index(row, col));
    }

    pub fn at_or_default(&self, row: usize, col: usize) -> T {
        match self.at(row, col) {
            Some(val) => val.to_owned(),
            None => Default::default(),
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<&mut Self, &str> {
        let index = self.index(row, col);

        if let Some(val) = self.matrix.get_mut(index) {
            *val = value;
            Ok(self)
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn apply<F>(&mut self, row: usize, col: usize, map: F) -> Result<&mut Self, &str>
    where
        F: Fn(&T) -> T,
    {
        let val = self.at(row, col);

        return match val {
            Some(v) => self.set(row, col, map(v)),
            None => Err("Index out of bounds"),
        };
    }

    pub fn map<F, TResult>(&self, map: F) -> Matrix<TResult>
    where
        F: Fn(&T) -> TResult,
        TResult: Default,
    {
        let result = self.matrix.iter().map(map).collect();

        return Matrix {
            rows: self.rows,
            cols: self.cols,
            matrix: result,
        };
    }

    pub fn rows(&self) -> Chunks<T> {
        return self.matrix.chunks(self.cols);
    }

    pub fn get_row(&self, i: usize) -> Option<&[T]> {
        return self.rows().nth(i);
    }
}

#[allow(dead_code)]
impl<Q> Matrix<Q>
where
    Q: Default + Copy + Clone,
    Q: Add<Output = Q> + Sub<Output = Q> + Mul<Output = Q> + Div<Output = Q>,
    for<'a> &'a Q: Add<Output = Q> + Sub<Output = Q> + Mul<Output = Q> + Div<Output = Q>,
{
    pub fn add(&self, value: Q) -> Matrix<Q> {
        return self.map(|x| *x + value);
    }

    pub fn subtract(&self, value: Q) -> Matrix<Q> {
        return self.map(|x| *x - value);
    }

    pub fn multiply(&self, value: Q) -> Matrix<Q> {
        return self.map(|x| *x * value);
    }

    pub fn matrix_add(&self, m: &Matrix<Q>) -> Option<Matrix<Q>> {
        if self.rows != m.rows || self.cols != m.cols {
            return None;
        }

        let mut result = Matrix::new(self.rows, self.cols);
        let result_iter = zip(self.matrix.iter(), m.matrix.iter());

        result_iter
            .map(|(x, y)| x + y)
            .enumerate()
            .for_each(|(i, z)| {
                if let Some(num) = result.matrix.get_mut(i) {
                    *num = z;
                }
            });

        return Some(result);
    }

    pub fn matrix_multiply(&self, m: &Matrix<Q>) -> Option<Matrix<Q>> {
        if self.rows != m.rows || self.cols != m.cols {
            return None;
        }

        let mut result = Matrix::new(self.rows, m.num_cols());
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                for k in 0..m.num_rows() {
                    let prod = self.at_or_default(i, k) * m.at_or_default(k, j);
                    let _ = result.apply(i, j, |x| x + &prod);
                }
            }
        }

        return Some(result);
    }
}

impl<D> fmt::Display for Matrix<D>
where
    D: Display + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("\n");
        let max_len = self
            .matrix
            .iter()
            .map(|x| (*x).to_string().len())
            .max()
            .unwrap();

        let rows = self.matrix.as_slice().chunks(self.cols);

        for row in rows {
            result.push_str("[ ");
            row.iter()
                .map(|x| {
                    let str = (*x).to_string();
                    let padded_str = format!("{:>max_len$}", str);

                    return padded_str;
                })
                .for_each(|x| result.push_str(&format!("{} ", x)));
            result.push_str("]\n");
        }

        return write!(f, "{}", result);
    }
}
