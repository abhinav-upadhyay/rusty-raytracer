use super::ray::Ray;

pub trait Intersect<T> {
    fn intersect(&self, ray: &Ray) -> Intersection<T>;
}

pub struct Intersection<'a, T> {
    object: &'a T,
    points: Vec<f32>
}

impl<'a, T> Intersection<'a, T> {

    pub fn new(object: &'a T, points: Vec<f32>) -> Self {
        Self {object, points}
    }

    pub fn object(&self) -> &T {
        self.object
    }

    pub fn points(&self) -> &Vec<f32> {
        &self.points
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::sphere::Sphere;

    #[test]
    fn test_intersection_constructor() {
        let s = Sphere::new(1);
        let i: Intersection<Sphere> = Intersection::new(&s, vec![3.1]);
        assert_eq!(i.object(), &Sphere::new(1));
        assert_eq!(i.points(), &vec![3.1]);
    }

}
