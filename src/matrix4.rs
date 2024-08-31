use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    data: [T; 4],
}

impl<T: Clone> Matrix<T> {
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        Matrix { data: data.to_owned() }
    }

    pub fn by_row(&self) -> Vec<Cell<T>> {
        self.data.iter().cloned().map(Cell).collect()
    }

    pub fn by_col(&self) -> Vec<Cell<T>> {
        [&self.data[0], &self.data[2], &self.data[1], &self.data[3]].
            into_iter().cloned().map(Cell).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, rhs: Cell<String>) -> Self::Output {
        if self.0 < 0 {
            let string = rhs.0.chars().rev().collect::<String>();
            let number = self.0.abs();

            Cell(format!("{} {}", string, number))
        } else {
            let number = self.0;
            let string = rhs.0;

            Cell(format!("{} {}", number, string))
        }
    }
}

impl Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, rhs: Cell<String>) -> Self::Output {
        let mut number = self.0;
        let mut string = rhs.0;

        if self.0 < 0 {
            string = string.chars().rev().collect();
            number = number.abs();
        }

        let mut output = String::new();

        for _ in 0..number {
            output.push_str(&string);
        }

        Cell(output)
    }
}

impl Add<Matrix<String>> for Matrix<i32> {
    type Output = Matrix<String>;

    fn add(self, rhs: Matrix<String>) -> Self::Output {
        let left_rows = self.by_row().into_iter();
        let right_rows = rhs.by_row().into_iter();

        let sums: Vec<String> = left_rows.zip(right_rows).map(|(l, r)| (l + r).0).collect();
        Matrix::new(&sums.try_into().unwrap())
    }
}

impl Mul<Matrix<String>> for Matrix<i32> {
    type Output = String;

    fn mul(self, rhs: Matrix<String>) -> Self::Output {
        let left_rows = self.by_row().into_iter();
        let right_cols = rhs.by_col().into_iter();

        let products: Vec<String> = left_rows.zip(right_cols).map(|(l, r)| (l * r).0).collect();
        products.join(" ")
    }
}
