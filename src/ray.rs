use super::tuple::Tuple;
use super::matrix::Matrix;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        //TODO: first parameter should be a point and second a vector
        // do we need to add a check?
        Self {origin, direction}
    }

    pub fn origin(&self) -> &Tuple {
        &self.origin
    }

    pub fn direction(&self) -> &Tuple {
        &self.direction
    }

    pub fn position(&self, t: f32) -> Tuple {
        (self.direction * t) + self.origin
    }

    pub fn transform(&self, transform: &Matrix) -> Self {
        return Self {origin: (transform * &self.origin).unwrap(),
            direction: (transform * &self.direction).unwrap()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::transform::TransformBuilder;

    #[test]
    fn test_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 1.0), Tuple::vector(3.0, 4.0, 5.0));
        assert_eq!(r.origin(), Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(Tuple::vector(3.0, 4.0, 5.0), r.direction());
    }

    #[test]
    fn test_position() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_ray_translation() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let transform = TransformBuilder::new(4).translate(3.0, 4.0, 5.0).build();
        let r2 = r.transform(&transform);
        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_ray_scaling() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let transform = TransformBuilder::new(4).scale(2.0, 3.0, 4.0).build();
        let r2 = r.transform(&transform);
        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }
}
