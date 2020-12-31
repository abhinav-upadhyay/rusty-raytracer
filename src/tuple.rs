use std::ops::{Add, Sub, Neg, Mul, Div};

//TODO: maybe we can just create two structs point and vector
// that will allow us to enforce the addition/subtraction limits
// for example point + point doesn't make sense and should not be allowed
#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}


impl Tuple {
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z, w: 1.0f32}
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z, w: 0.0}
    }

    pub fn magnitude(&self) -> f32 {
        let sq_sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        sq_sum.sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        return self / magnitude;
    }

    pub fn dot(&self, other :&Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

}

impl Add<Tuple> for Tuple {
    type Output = Self;
    fn add(self, _rhs: Tuple) -> Self {
        Self {x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z, w: self.w + _rhs.w}
    }
}

impl Add<&Tuple> for Tuple {
    type Output = Self;
    fn add(self, _rhs: &Tuple) -> Self {
        Self {x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z, w: self.w + _rhs.w}
    }
}


impl Sub<Tuple> for Tuple {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self {
        Self{x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z, w: self.w - _rhs.w}
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Self{x: -self.x, y: -self.y, z: -self.z, w: self.w}
    }

}

impl Mul<f32> for Tuple {
    type Output = Self;
    fn mul(self, _rhs: f32) -> Self {
        Self{x:  self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs, w: self.w}
    }
}

impl Mul<Tuple> for Tuple {
    type Output = Self;
    fn mul(self, _rhs: Tuple) -> Self {
        Self {x: self.y * _rhs.z - self.z * _rhs.y, y: self.z * _rhs.x - self.x * _rhs.z, z: self.x * _rhs.y - self.y * _rhs.x, w: self.w}
    }
}

impl Div<f32> for Tuple {
    type Output = Self;
    fn div(self, _rhs: f32) -> Self {
        Self{x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs, w: self.w}
    }
}


impl Div<i32> for Tuple {
    type Output = Self;
    fn div(self, _rhs: i32) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

impl Div<i64> for Tuple {
    type Output = Self;
    fn div(self, _rhs: i64) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

impl Div<i16> for Tuple {
    type Output = Self;
    fn div(self, _rhs: i16) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

impl Div<i8> for Tuple {
    type Output = Self;
    fn div(self, _rhs: i8) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

impl Div<u32> for Tuple {
    type Output = Self;
    fn div(self, _rhs: u32) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

impl Div<u64> for Tuple {
    type Output = Self;
    fn div(self, _rhs: u64) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

impl Div<u16> for Tuple {
    type Output = Self;
    fn div(self, _rhs: u16) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

impl Div<u8> for Tuple {
    type Output = Self;
    fn div(self, _rhs: u8) -> Self {
        let den = _rhs as f32;
        Self{x: self.x / den, y: self.y / den, z: self.z / den, w: self.w}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_floats(x :f32, y: f32) -> bool {
        (x - y).abs() <= 1e-3
    }

    #[test]
    fn test_point() {
        let p = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.w, 1.0);
    }

    #[test]
    fn test_vector() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.w, 0.0);
    }

    #[test]
    fn test_point_eq_pos() {
        let p1 = Tuple::point(1.0, 2.0, 3.0);
        let p2 = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(p1, p2);
    }
    
    #[test]
    fn test_vector_eq_pos() {
        let p1 = Tuple::vector(1.0, 2.0, 3.0);
        let p2 = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_point_vector_add() {
        let p1 = Tuple::point(3 as f32, -2 as f32, 5 as f32);
        let v1 = Tuple::vector(-2 as f32, 3 as f32, 1 as f32);
        let result = p1 + v1;
        assert_eq!(result, Tuple::point(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_vector_vector_add() {
        let v1 = Tuple::vector(3.0, 2.0, 5.0);
        let v2 = Tuple::vector(-2.0, 3.0, 2.0);
        let result = v1 + v2;
        assert_eq!(result, Tuple::vector(1.0, 5.0, 7.0));
    }

    #[test]
    fn test_point_point_sub() {
        let p1 = Tuple::point(3 as f32, 2 as f32, 1 as f32);
        let p2 = Tuple::point(5 as f32, 6 as f32, 7 as f32);
        let result = p1 - p2;
        assert_eq!(result, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_point_vector_sub() {
        let p = Tuple::point(3 as f32, 2 as f32, 1 as f32);
        let v = Tuple::vector(5 as f32, 6 as f32, 7 as f32);
        let result = p - v;
        assert_eq!(result, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_vector_vector_sub() {
        let v1 = Tuple::vector(3 as f32, 2 as f32, 1 as f32);
        let v2 = Tuple::vector(5 as f32, 6 as f32, 7 as f32);
        let result = v1 - v2;
        assert_eq!(result, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_vector_neg() {
        let v = Tuple::vector(3.0, 2.0, 1.0);
        assert_eq!(-v, Tuple::vector(-3.0, -2.0, -1.0))
    }

    #[test]
    fn test_float_scaler_mul() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        let result = p * 3.5;
        assert_eq!(result, Tuple::vector(3.5, 7.0, 10.5));
    }

    #[test]
    fn test_int_scaler_mul() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        let result = p * 2 as f32;
        assert_eq!(result, Tuple::vector(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_fraction_mul() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        let result = p * 0.5;
        assert_eq!(result, Tuple::vector(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_f32_div() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        let result = p / 0.5;
        assert_eq!(result, Tuple::vector(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_i32_div() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        let result = p / 2;
        assert_eq!(result, Tuple::vector(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_u32_div() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        let result = p / 2u32;
        assert_eq!(result, Tuple::vector(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_u64_div() {
        let p = Tuple::vector(1.0, 2.0, 3.0);
        let result = p / 2u64;
        assert_eq!(result, Tuple::vector(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_unit_mag1() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_unit_mag2() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_unit_mag3() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_magnitude1() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14f32.sqrt());
    }

    #[test]
    fn test_magnitude2() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14f32.sqrt());
    }

    #[test]
    fn test_normalize1() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normalize2() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0 / 14.0f32.sqrt(), 2.0/14.0f32.sqrt(), 3.0/14.0f32.sqrt()))
    }

    #[test]
    fn test_normalize_magnitude() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let normalized = v.normalize();
        assert_eq!(true, compare_floats(normalized.magnitude(), 1.0f32));
    }

    #[test]
    fn test_dot() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn test_cross_product1() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v1 * v2, Tuple::vector(-1.0, 2.0, -1.0));
    }

    #[test]
    fn test_cross_product2() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v2 * v1, Tuple::vector(1.0, -2.0, 1.0));
    }

}