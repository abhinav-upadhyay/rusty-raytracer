use super::ray::Ray;
use super::tuple::Tuple;
use super::matrix::Matrix;
use super::material::Material;
use super::intersection::{Intersect, Intersection, Intersections};


#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    transform: Matrix,
    pub material: Material,
    id: i32
}

impl Sphere {
    pub fn new(id: i32) -> Self {
        Self{id, transform: Matrix::identity(4), material: Material::new()}
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

    fn normal_at(&self, point: Tuple) -> Tuple {
        let transform_inverse = self.transform.inverse().unwrap();
        let object_point = (&transform_inverse * &point).unwrap();
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let world_normal = (transform_inverse.transpose() * object_normal).unwrap();
        let world_normal_vector = Tuple::vector(world_normal.x(), world_normal.y(), world_normal.z());
        return world_normal_vector.normalize();
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

    #[test]
    fn test_normal_at_x() {
        let s = Sphere::new(1);
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normal_at_y() {
        let s = Sphere::new(1);
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_normal_at_z() {
        let s= Sphere::new(1);
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normal_at_nonaxial() {
        let s = Sphere::new(1);
        let v = 3.0f32.sqrt() / 3.0;
        let n = s.normal_at(Tuple::point(v, v, v));
        assert_eq!(n, Tuple::vector(v, v, v));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_normal_translated_sphere() {
        let mut s = Sphere::new(1);
        s.set_transform(TransformBuilder::new(4).translate(0.0, 1.0, 0.0).build());
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_normal_transformed_sphere() {
        let mut s = Sphere::new(1);
        s.set_transform(TransformBuilder::new(4).rotate_z(std::f32::consts::PI / 5.0).scale(1.0, 0.5, 1.0).build());
        let v = 2.0f32.sqrt() / 2.0;
        let n = s.normal_at(Tuple::point(0.0, v, v));
        assert_eq!(n, Tuple::vector(0.0, 0.97014254, 0.24253564));
    }

    #[test]
    fn test_sphere_default_material() {
        let s = Sphere::new(1);
        let m = s.material;
        assert_eq!(m, Material::new())
    }

    #[test]
    fn test_sphere_matterial_assignment() {
        let mut s = Sphere::new(1);
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

}