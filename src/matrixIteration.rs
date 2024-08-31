#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Self {
        Matrix { rows, cols, data: data[0 .. (rows * cols)].to_vec() }
    }

    pub fn by_row(&self) -> RowIter<T> {
        RowIter { matrix: self, current: 0 }
    }

    pub fn by_col(&self) -> ColIter<T> {
        ColIter { matrix: self, current: 0 }
    }
}

pub struct RowIter<'a, T> {
    matrix: &'a Matrix<T>,
    current: usize,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.matrix.data.get(self.current);
        self.current += 1;
        item
    }
}

pub struct ColIter<'a, T> {
    matrix: &'a Matrix<T>,
    current: usize,
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_row = self.current % self.matrix.rows;
        let current_col = self.current / self.matrix.rows;
        if current_col >= self.matrix.cols {
            return None;
        }

        let item = self.matrix.data.get(current_row * self.matrix.cols + current_col);
        self.current += 1;
        item
    }
}
