use std::ops::{Index, Mul};
use std::cmp::PartialEq;
use super::utils;
use super::tuple::Tuple;

#[derive(Debug, Clone)]
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

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::new(size, size);
        for i in 0..size {
            matrix.set(i, i, 1.0);
        }
        matrix
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

    pub fn transpose(&self) -> Self {
        let mut transpose = Self::new(self.nrows, self.ncols);
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                transpose.set(j, i, self[(i, j)]);
            }
        }
        transpose
    }

    //TODO: determinant is only defined for square matrices, maybe we need to return a Result?
    pub fn det(&self) -> f32 {
        if self.nrows == 2 && self.ncols == 2 {
            return self.vals[0] * self.vals[3] - self.vals[1] * self.vals[2]
        }
        let rowid = 0;
        let mut det = 0.0;
        for i in 0..self.ncols {
            det += self[(rowid, i)] * self.cofactor(rowid, i);
        }
        return det;
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut submat = Matrix::new(self.nrows - 1, self.ncols - 1);
        let mut rowid = 0;
        for i in 0..self.nrows {
            let mut colid = 0;
            if i == row {
                continue;
            }
            for j in 0..self.ncols {
                if j == col {
                    continue;
                }
                submat.set(rowid, colid, self[(i, j)]);
                colid += 1;
            }
            rowid += 1;
        }
        return submat;
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let subm = self.submatrix(row, col);
        return subm.det();
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor_val = self.minor(row, col);
        if (row + col) % 2 != 0 {
            return -minor_val;
        }
        minor_val
    }

    pub fn is_invertible(&self) -> bool {
        self.det() != 0.0
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }
        let mut inverse_mat = Self::new(self.nrows, self.ncols);
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                let c = self.cofactor(i, j);
                inverse_mat.set(j, i, c / det);
            }
        }
        Some(inverse_mat)
    }

    //specialized for 4 x 4 dimensions since raytracer requires only 3 dimensions
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut ident = Self::identity(4);
        ident.set(0, 3, x);
        ident.set(1, 3, y);
        ident.set(2, 3, z);
        return ident;
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        let mut ident = Self::identity(4);
        ident.set(0, 0, x);
        ident.set(1, 1, y);
        ident.set(2, 2, z);
        return ident;
    }

    pub fn rotation_x(angle: f32) -> Self {
        Self::from_array(4, 4, &[1.0, 0.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(), 0.0,
            0.0, angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0]).unwrap()
    }

    pub fn rotation_y(angle: f32) -> Self {
        Self::from_array(4, 4, &[angle.cos(), 0.0, angle.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -angle.sin(), 0.0, angle.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0]).unwrap()
    }

    pub fn rotation_z(angle: f32) -> Self {
        Self::from_array(4, 4, &[angle.cos(), -angle.sin(), 0.0, 0.0,
        angle.sin(), angle.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0]).unwrap()
    }

    pub fn shearing(x_y :f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Self {
        Self::from_array(4, 4, &[1.0, x_y, x_z, 0.0,
            y_x, 1.0, y_z, 0.0,
            z_x, z_y, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0]).unwrap()
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;
    fn index(&self, indexer: (usize, usize)) -> &Self::Output {
        let (row, col) = indexer;
        let idx = row * self.ncols + col;
        return &self.vals[idx];
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Option<Matrix>;
    fn mul(self, _rhs: &Matrix) -> Option<Matrix> {
        if self.ncols != _rhs.nrows {
            return None;
        }
        let mut prod = Matrix::new(self.nrows, _rhs.ncols);
        for i in 0..self.nrows {
            for j in 0.._rhs.ncols {
                let mut cell_val = 0.0f32;
                for k in 0..self.ncols {
                    cell_val += self[(i, k)] * _rhs[(k, j)];
                }
                prod.set(i, j,cell_val);
            }
        }
        return Some(prod);
    }
}


impl Mul<Matrix> for Matrix {
    type Output = Option<Matrix>;
    fn mul(self, _rhs: Self) -> Option<Matrix> {
        if self.ncols != _rhs.nrows {
            return None;
        }
        let mut prod = Matrix::new(self.nrows, _rhs.ncols);
        for i in 0..self.nrows {
            for j in 0.._rhs.ncols {
                let mut cell_val = 0.0f32;
                for k in 0..self.ncols {
                    cell_val += self[(i, k)] * _rhs[(k, j)];
                }
                prod.set(i, j,cell_val);
            }
        }
        return Some(prod);
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Option<Tuple>;
    fn mul(self, _rhs: &Tuple) -> Option<Tuple> {
        if self.ncols != 4 {
            return None;
        }
        let rhs_vals  = [_rhs.x(), _rhs.y(), _rhs.z(), _rhs.w()];
        let b = Matrix::from_array(self.nrows, 1, &rhs_vals).unwrap();
        let prod = (self * &b).unwrap();
        return Some(Tuple::new(prod[(0, 0)], prod[(1, 0)], prod[(2, 0)], prod[(3, 0)]));
    }
}


impl Mul<Tuple> for Matrix {
    type Output = Option<Tuple>;
    fn mul(self, _rhs: Tuple) -> Option<Tuple> {
        if self.ncols != 4 {
            return None;
        }
        let rhs_vals  = [_rhs.x(), _rhs.y(), _rhs.z(), _rhs.w()];
        let b = Matrix::from_array(self.nrows, 1, &rhs_vals).unwrap();
        let prod = (self * b).unwrap();
        return Some(Tuple::new(prod[(0, 0)], prod[(1, 0)], prod[(2, 0)], prod[(3, 0)]));
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
    use std::f32::consts::PI;
    use super::*;
    const SQRT_TWO: f32 = 1.4142135623730951_f32;

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

    #[test]
    fn test_matrix_mul() {
        let a = Matrix::from_array(4, 4, &[1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0]).unwrap();
        let b = Matrix::from_array(4, 4, &[-2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0]).unwrap();
        let expected_prod = Matrix::from_array(4, 4, &[20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0]).unwrap();
        assert_eq!((a * b).unwrap(), expected_prod);
    }

    #[test]
    fn test_matrix_vector_mul() {
        let a = Matrix::from_array(4, 4, &[1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0]).unwrap();
        let b = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!((a * b).unwrap(), Tuple::point(18.0, 24.0, 33.0));
    }

    #[test]
    fn test_matrix_identity_mul() {
        let a = Matrix::from_array(4, 4, &[0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0]).unwrap();
        assert_eq!((&a * &Matrix::identity(4)).unwrap(), a);
    }

    #[test]
    fn test_tuple_identity_mul() {
        let ident = Matrix::identity(4);
        let tup = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!((&ident * &tup).unwrap(), tup);
    }

    #[test]
    fn test_transpose() {
        let a = Matrix::from_array(4, 4, &[0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0., 5.0, 8.0]).unwrap();
        let expected_transpose = Matrix::from_array(4, 4, &[0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0]).unwrap();
        assert_eq!(a.transpose(), expected_transpose);
    }

    #[test]
    fn test_identity_transpose() {
        let ident = Matrix::identity(4);
        assert_eq!(ident.transpose(), ident);
    }

    #[test]
    fn test_det() {
        let a = Matrix::from_array(2, 2, &[1.0, 5.0, -3.0, 2.0]).unwrap();
        assert_eq!(a.det(), 17.0);
    }

    #[test]
    fn test_submat1() {
        let a = Matrix::from_array(3, 3, &[1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0]).unwrap();
        let expected_submat = Matrix::from_array(2, 2, &[-3.0, 2.0, 0.0, 6.0]).unwrap();
        assert_eq!(a.submatrix(0, 2), expected_submat);
    }

    #[test]
    fn test_submat2() {
        let a = Matrix::from_array(4, 4, &[-6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0]).unwrap();
        let expected_sumat = Matrix::from_array(3, 3, &[-6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0]).unwrap();
        assert_eq!(a.submatrix(2, 1), expected_sumat);
    }

    #[test]
    fn test_minor() {
        let a = Matrix::from_array(3, 3, &[3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0]).unwrap();
        let b = a.submatrix(1, 0);
        assert_eq!(b.det(), a.minor(1, 0));
    }

    #[test]
    fn test_cofactor() {
        let a = Matrix::from_array(3, 3, &[3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0]).unwrap();
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_det_3_by_3() {
        let a = Matrix::from_array(3, 3, &[1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0]).unwrap();
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.det(), -196.0);
    }

    #[test]
    fn test_det_4_by_4() {
        let a = Matrix::from_array(4, 4, &[-2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0]).unwrap();
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.det(), -4071.0);
    }

    #[test]
    fn test_invertibility() {
        let mat1 = Matrix::from_array(4, 4, &[6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0]).unwrap();
        assert_eq!(mat1.is_invertible(), true);

        let mat2 = Matrix::from_array(4, 4, &[-4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0]).unwrap();
        assert_eq!(mat2.is_invertible(), false);
    }

    #[test]
    fn test_inverse1() {
        let a = Matrix::from_array(4, 4, &[-5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0]).unwrap();
        let b = a.inverse().unwrap();
        let expected_inverse = Matrix::from_array(4, 4, &[
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639 
        ]).unwrap();
        assert_eq!(a.det(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[(2, 3)], 105.0 / 532.0);
        assert_eq!(b, expected_inverse);
    }

    #[test]
    fn test_inverse2() {
        let a = Matrix::from_array(4, 4, &[8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0]).unwrap();
        let expected_inverse = Matrix::from_array(4, 4, &[-0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308]).unwrap();
        assert_eq!(a.inverse().unwrap(), expected_inverse);
    }

    #[test]
    fn test_inverse3() {
        let a = Matrix::from_array(4, 4, &[9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0]).unwrap();
        let expected_inverse = Matrix::from_array(4, 4, &[-0.040704, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333]).unwrap();
        assert_eq!(a.inverse().unwrap(), expected_inverse);
    }

    #[test]
    fn test_inverse4() {
        let a = Matrix::from_array(4, 4, &[3.0, -9.0, 7.0, 3.0,
            3.0, -8.0, 2.0, -9.0,
            -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0]).unwrap();
        let b = Matrix::from_array(4, 4, &[8.0, 2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0, 0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 5.0]).unwrap();
        let c = (&a * &b).unwrap();
        let inv = b.inverse().unwrap();
        assert_eq!((c * inv).unwrap(), a);
    }

    #[test]
    fn test_translate() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_inverse_translate() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!((inv * p).unwrap(), Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_translate_vector() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!((&transform * &v).unwrap(), v);
    }

    #[test]
    fn test_scaling_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scaling_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!((transform * v).unwrap(), Tuple::vector(-8.0, 18.0, 32.0));

    }

    #[test]
    fn test_inverse_scaling() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let inv = transform.inverse().unwrap();
        assert_eq!((inv * v).unwrap(), Tuple::vector(-2.0, 2.0, 2.0));

    }

    #[test]
    fn test_reflection() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotation_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        assert_eq!((&half_quarter * &p).unwrap(), Tuple::point(0.0, SQRT_TWO / 2f32, 2f32.sqrt() / 2f32));
        assert_eq!((&full_quarter * &p).unwrap(), Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotation_x_inverse() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let rotation_inv = half_quarter.inverse().unwrap();
        assert_eq!((rotation_inv * p).unwrap(), Tuple::point(0.0, SQRT_TWO / 2.0, -SQRT_TWO / 2.0));
    }

    #[test]
    fn test_rotation_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        assert_eq!((&half_quarter * &p).unwrap(), Tuple::point(SQRT_TWO / 2f32, 0.0, SQRT_TWO / 2f32));
        assert_eq!((&full_quarter * &p).unwrap(), Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotation_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        assert_eq!((&half_quarter * &p).unwrap(), Tuple::point(-SQRT_TWO / 2f32, 2f32.sqrt() / 2f32, 0.0));
        assert_eq!((&full_quarter * &p).unwrap(), Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing_x_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_x_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_y_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shearing_y_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_z_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_z_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!((transform * p).unwrap(), Tuple::point(2.0, 3.0, 7.0));
    }

}