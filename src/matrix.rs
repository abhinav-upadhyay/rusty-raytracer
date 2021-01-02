use std::ops::Index;
use std::cmp::PartialEq;
use super::utils;

#[derive(Debug)]
pub struct Matrix {
    nrows: usize,
    ncols: usize,
    vals: Vec<f32>
}

impl Matrix {
    
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let vals = vec![0.0; nrows * ncols];
        Self {nrows, ncols, vals}
    }

    pub fn from_array(nrows: usize, ncols: usize, vals: &[f32]) -> Option<Self> {
        if vals.len() != nrows * ncols {
            return None;
        }
        let vals_vec = Vec::from(vals);
        return Some(Self {nrows, ncols, vals: vals_vec});
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.nrows || col >= self.ncols {
            return None;
        }
        Some(row * self.ncols + col)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        match self.get_index(row, col) {
            None => return None,
            Some(index) => Some(self.vals[index])
        }
    }

    //TODO: maybe a performant impl would not return anything
    pub fn set(&mut self, row: usize, col: usize, val: f32) -> Option<f32> {
        match self.get_index(row, col) {
            None => None,
            Some(index) => {
                let old_val = self.vals[index];
                self.vals[index] = val;
                Some(old_val)
            }
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;
    fn index(&self, indexer: (usize, usize)) -> &Self::Output {
        let (row, col) = indexer;
        let idx = row * self.nrows + col;
        return &self.vals[idx];
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.shape() != other.shape() {
            return false;
        }
        for (i, v) in self.vals.iter().enumerate() {
            if !utils::is_equal(*v, other.vals[i]) {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_init_from_arr() {
        let mat = Matrix::from_array(4, 4, &[1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5]).unwrap();
        assert_eq!(mat.shape(), (4, 4));
        assert_eq!(mat.get(0, 0).unwrap(), 1.0);
        assert_eq!(mat.get(0, 3).unwrap(), 4.0);
        assert_eq!(mat.get(1, 0).unwrap(), 5.5);
        assert_eq!(mat.get(1, 2).unwrap(), 7.5);
        assert_eq!(mat.get(2, 2).unwrap(), 11.0);
        assert_eq!(mat.get(3, 0).unwrap(), 13.5);
        assert_eq!(mat.get(3, 2).unwrap(), 15.5);
        assert_eq!(mat[(0, 0)], 1.0);
        assert_eq!(mat[(0, 3)], 4.0);
        assert_eq!(mat[(1, 0)], 5.5);
        assert_eq!(mat[(1, 2)], 7.5);
        assert_eq!(mat[(2, 2)], 11.0);
        assert_eq!(mat[(3, 0)], 13.5);
        assert_eq!(mat[(3, 2)], 15.5);
    }

    #[test]
    fn test_init_2_by_2() {
        let mat = Matrix::from_array(2, 2, &[-3.0, 5.0, 1.0, -2.0]).unwrap();
        assert_eq!(mat[(0, 0)], -3.0);
        assert_eq!(mat[(0, 1)], 5.0);
        assert_eq!(mat[(1, 0)], 1.0);
        assert_eq!(mat[(1, 1)], -2.0);
    }

    #[test]
    fn test_3_by_3() {
        let mat = Matrix::from_array(3, 3, &[-3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 5.0, 1.0]).unwrap();
        assert_eq!(mat[(0, 0)], -3.0);
        assert_eq!(mat[(1, 1)], -2.0);
        assert_eq!(mat[(2, 2)], 1.0);
    }

    #[test]
    fn test_matrix_equality_positive() {
        let mat1 = Matrix::from_array(4, 4, &[1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0]).unwrap();
        let mat2 = Matrix::from_array(4, 4, &[1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0]).unwrap();
        assert_eq!(mat1, mat2);
    }

    #[test]
    fn test_matrix_equality_neg1() {
        let mat1 = Matrix::from_array(4, 4, &[1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0]).unwrap();
        let mat2 = Matrix::from_array(3, 4, &[1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0]).unwrap();
        assert_ne!(mat1, mat2);
    }

    #[test]
    fn test_matrix_equality_neg2() {
        let mat1 = Matrix::from_array(4, 4, &[1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0]).unwrap();
        let mat2 = Matrix::from_array(4, 4, &[2.0, 3.0, 4.0, 5.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0]).unwrap();
        assert_ne!(mat1, mat2);
    }
    

}