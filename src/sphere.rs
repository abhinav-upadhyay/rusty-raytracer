use super::ray::Ray;
use super::tuple::Tuple;
use super::matrix::Matrix;
use super::intersection::{Intersect, Intersection, Intersections};


#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    transform: Matrix,
    id: i32
}

impl Sphere {
    pub fn new(id: i32) -> Self {
        Self{id, transform: Matrix::identity(4)}
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

}

impl Intersect<Self> for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersections<Self> {
        let transformed_ray = ray.transform(&self.transform.inverse().unwrap());
        let sphere_to_ray = transformed_ray.origin() - &Tuple::point(0.0, 0.0, 0.0);
        let a = transformed_ray.direction().dot(transformed_ray.direction());
        let b = 2.0 * transformed_ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::new_empty();
        }
        let mut xs = Intersections::with_capacity(2);
        let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b + discriminant.sqrt()) / (2.0 * a);
        if x1 < x2 {
            xs.add_point(Intersection::new(self, x1));
            xs.add_point(Intersection::new(self, x2));
        } else {
            xs.add_point(Intersection::new(self, x2));
            xs.add_point(Intersection::new(self, x1));
        }
        return xs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::transform::TransformBuilder;

    #[test]
    fn test_ray_intersection() {
        let s = Sphere::new(1);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = s.intersect(&r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].point(), 4.0);
        assert_eq!(intersections[1].point(), 6.0);
    }

    #[test]
    fn test_tangent_intersection() {
        let s = Sphere::new(1);
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = s.intersect(&r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].point(), 5.0);
        assert_eq!(intersections[1].point(), 5.0);
    }

    #[test]
    fn test_no_intersection() {
        let s = Sphere::new(1);
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = s.intersect(&r);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_inside_ray_intersection() {
        //ray originates from within the sphere - we still get two intersections
        let s = Sphere::new(1);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = s.intersect(&r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].point(), -1.0);
        assert_eq!(intersections[1].point(), 1.0);
    }

    #[test]
    fn test_intersection_behind() {
        // if the sphere lies behind the ray, even then we get two intersections with negative values
        let s = Sphere::new(1);
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = s.intersect(&r);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].point(), -6.0);
        assert_eq!(intersections[1].point(), -4.0);
    }

    #[test]
    fn test_transform() {
        let s = Sphere::new(1);
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn test_set_transform() {
        let mut s = Sphere::new(1);
        let transform = TransformBuilder::new(4).translate(2.0, 3.0, 4.0).build();
        s.set_transform(transform.clone());
        assert_eq!(s.transform, transform);
    }

    #[test]
    fn test_scaled_sphere_ray_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new(1);
        s.set_transform(TransformBuilder::new(4).scale(2.0, 2.0, 2.0).build());
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].point(), 3.0);
        assert_eq!(xs[1].point(), 7.0);
    }

    #[test]
    fn test_translated_sphere_ray_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new(1);
        s.set_transform(TransformBuilder::new(4).translate(5.0, 0.0, 0.0).build());
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

}