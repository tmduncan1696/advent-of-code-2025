use std::convert::From;

use crate::day04::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let mut start = Matrix::from(input);

    let nrow = start.get_nrow();
    let ncol = start.get_ncol();

    let mut num_removed = 1;
    let mut out = 0;

    while num_removed > 0 {
        let sum_mat = start.sum_surrounding();

        let data_zip = sum_mat
            .get_data()
            .into_iter()
            .zip(start.get_data());

        let new_data: Vec<usize> = data_zip
            .clone()
            .map(|(a, b)| if *a < 4 { 0 } else { *b })
            .collect();

        num_removed = data_zip
            .filter(|(a, b)| **a < 4 && **b == 1)
            .count();

        out += num_removed;

        start = Matrix::new(
            (nrow, ncol),
            new_data
        );
    }

    Some(out)
}

struct Matrix {
    nrow: usize,
    ncol: usize,
    data: Vec<usize>
}

impl Matrix {
    fn new(size: (usize, usize), data: Vec<usize>) -> Self {
        Matrix {
            nrow: size.0,
            ncol: size.1,
            data
        }
    }


    fn get_nrow(&self) -> usize {
        self.nrow
    }

    fn get_ncol(&self) -> usize {
        self.ncol
    }

    fn get_data(&self) -> &Vec<usize> {
        &self.data
    }

    fn get_value(&self, row: usize, col: usize) -> Option<&usize> {
        if row >= self.get_nrow() || col >= self.get_ncol() {
            return None
        };

        self.get_data().get(row * self.get_ncol() + col)
    }

    fn sum_surrounding_elements(&self, row: usize, col: usize) -> usize {
        let mut out = 0;

        if row != 0 && col != 0 {
            out += self.get_value(row - 1, col - 1).unwrap_or(&0);
        };

        if row != 0 {
            out += self.get_value(row - 1, col).unwrap_or(&0)
                + self.get_value(row -1, col + 1).unwrap_or(&0);
        };

        if col != 0 {
            out += self.get_value(row, col - 1).unwrap_or(&0)
                + self.get_value(row + 1, col - 1).unwrap_or(&0);

        }
        
        out += self.get_value(row, col + 1).unwrap_or(&0)
            + self.get_value(row + 1, col).unwrap_or(&0)
            + self.get_value(row + 1, col + 1).unwrap_or(&0);

        out
    }

    fn sum_surrounding(&self) -> Self {
        let nrow = self.get_nrow();
        let ncol = self.get_ncol();

        let mut out_vec = Vec::new();

        for i in 0..nrow {
            for j in 0..ncol {
                out_vec.push(self.sum_surrounding_elements(i, j));
            };
        };

        Matrix::new((nrow, ncol), out_vec)

    }
}

impl From<Input> for Matrix {
    fn from(input: Input) -> Self {
        let nrow = input.len();
        let ncol = input[0].len();
        let size = (nrow, ncol);

        let input_vec = input
            .to_vec()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        Matrix::new(size, input_vec)
    }
}

impl From<&Input> for Matrix {
    fn from(input: &Input) -> Self {
        let nrow = input.len();
        let ncol = input[0].len();
        let size = (nrow, ncol);

        let input_vec = input
            .to_vec()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        Matrix::new(size, input_vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;

   use crate::day04::input::read;

    #[test]
    fn test_solve() {
        let input_str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let input = read(input_str);
        assert_eq!(
            solve(&input),
            Some(43)
        );
    }
}
