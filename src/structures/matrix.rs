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

    pub fn get(&self, row: i32, col: i32) -> &T {

        &self.array[row as usize * self.cols + col as usize]
    }

    pub fn set(&mut self, row: i32, col: i32, value: T) {
        self.array[row as usize * self.cols + col as usize] = value;
    }
}