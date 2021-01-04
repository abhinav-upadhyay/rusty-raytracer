    use super::matrix::Matrix;
    pub struct TransformBuilder {
        matrix: Matrix
    }

    impl TransformBuilder {
        pub fn new(size: usize) -> Self {
            Self{matrix: Matrix::identity(size)}
        }

        pub fn rotate_x(self, angle: f32) -> Self {
            let result = (Matrix::rotation_x(angle) * self.matrix).unwrap();
            Self {matrix: result}
        }

        pub fn rotate_y(self, angle: f32) -> Self {
            let result = (Matrix::rotation_y(angle) * self.matrix).unwrap();
            Self {matrix: result}
        }

        pub fn rotate_z(self, angle: f32) -> Self {
            let result = (Matrix::rotation_z(angle) * self.matrix).unwrap();
            Self {matrix: result}
        }

        pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
            let result = (Matrix::scaling(x, y, z) * self.matrix).unwrap();
            Self {matrix: result}
        }

        pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
            let result = (Matrix::translation(x, y, z) * self.matrix).unwrap();
            Self {matrix: result}
        }

        pub fn shear(self, x_y :f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Self {
            let result = (Matrix::shearing(x_y, x_z, y_x, y_z, z_x, z_y) * self.matrix).unwrap();
            Self {matrix: result}
        }

        pub fn build(self) -> Matrix {
            self.matrix
        }

    }

    #[cfg(test)]
    mod tests {
        use std::f32::consts::PI;
        use super::*;
        use super::super::tuple::Tuple;

        #[test]
        fn test_chain_transformations() {
            let p = Tuple::point(1.0, 0.0, 1.0);
            let rotation = Matrix::rotation_x(PI/2.0);
            let scaling = Matrix::scaling(5.0, 5.0, 5.0);
            let translation = Matrix::translation(10.0, 5.0, 7.0);
            let p2 = (&rotation * &p).unwrap();
            assert_eq!(&p2, &Tuple::point(1.0, -1.0, 0.0));
            let p3 = (&scaling * &p2).unwrap();
            assert_eq!(&p3, &Tuple::point(5.0, -5.0, 0.0));
            let p4 = (&translation * &p3).unwrap();
            assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
            let transform = TransformBuilder::new(4).rotate_x(PI / 2.0).scale(5.0, 5.0, 5.0).translate(10.0, 5.0, 7.0).build();
            assert_eq!((transform * p).unwrap(), Tuple::point(15.0, 0.0, 7.0));
        }
    }
    