use super::ray::Ray;

pub trait Intersect<T> {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<T>>;
}

pub struct Intersection<'a, T> {
    object: &'a T,
    point: f32
}

impl<'a, T> Intersection<'a, T> {

    pub fn new(object: &'a T, point: f32) -> Self {
        Self {object, point}
    }

    pub fn object(&self) -> &T {
        self.object
    }

    pub fn point(&self) -> f32 {
        self.point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::sphere::Sphere;

    #[test]
    fn test_intersection_constructor() {
        let s = Sphere::new(1);
        let i: Intersection<Sphere> = Intersection::new(&s, 3.1);
        assert_eq!(i.object(), &Sphere::new(1));
        assert_eq!(i.point(), 3.1);
    }

}
