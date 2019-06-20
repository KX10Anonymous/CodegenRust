use std::ops::Index;
use std::ops::IndexMut;

pub struct Matrix<T> 
{
    array: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix<T> where T: Default + Clone {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix::<T> {
            array: vec![T::default();rows * cols], // TODO : try performance without default values set
            rows: rows,
            cols: cols,
        }
    }

    pub fn get_array(&self) -> &[T] {
        &self.array
    }
}

impl<T> Index<[usize; 2]> for Matrix<T>
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let row = index[0];
        let col = index[1];
        &self.array[row * self.cols + col]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T>
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let row = index[0];
        let col = index[1];
        &mut self.array[row * self.cols + col]
    }
}