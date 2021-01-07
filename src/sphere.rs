use super::ray::Ray;
use super::tuple::Tuple;
use super::intersection::{Intersect, Intersection};


#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: i32
}

impl Sphere {
    pub fn new(id: i32) -> Self {
        Self{id}
    }

}

impl Intersect<Self> for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<Self>> {
        let sphere_to_ray = ray.origin() - &Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }
        let mut xs = Vec::with_capacity(2);
        let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b + discriminant.sqrt()) / (2.0 * a);
        if x1 < x2 {
            xs.push(Intersection::new(self, x1));
            xs.push(Intersection::new(self, x2));
        } else {
            xs.push(Intersection::new(self, x2));
            xs.push(Intersection::new(self, x1));
        }
        return xs;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

}