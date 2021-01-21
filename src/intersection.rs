use std::cmp::{Ord, PartialEq, PartialOrd, Ordering, Eq, Reverse};
use std::ops::Index;
use std::collections::BinaryHeap;
use super::ray::Ray;
use super::utils;

pub trait Intersect<T: PartialEq + Clone> {
    fn intersect(&self, ray: &Ray) -> Intersections<T>;
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a, T: PartialEq + Clone> {
    object: &'a T,
    point: f32
}

pub struct Intersections<'a, T: PartialEq + Clone> {
    pos_intersections: BinaryHeap<Reverse<Intersection<'a, T>>>,
    neg_intersections: BinaryHeap<Reverse<Intersection<'a, T>>>,
    merged_intersections: Vec<Intersection<'a, T>>

}

impl<'a, T: PartialEq + Clone> Intersections<'a, T> {
    pub fn new(intersections: Vec<Intersection<'a, T>>) -> Self {
        let mut pos_intersections = BinaryHeap::with_capacity(intersections.len());
        let mut neg_intersections = BinaryHeap::with_capacity(intersections.len());
        let mut merged_intersections = Vec::with_capacity(intersections.len());
        for i in intersections {
            merged_intersections.push(i.clone());
            if i.point > 0.0 {
                pos_intersections.push(Reverse(i));
            } else {
                neg_intersections.push(Reverse(i));
            }
        }
        Self {pos_intersections, neg_intersections, merged_intersections}
    }

    pub fn new_empty() -> Self {
        Self {pos_intersections: BinaryHeap::new(),
            neg_intersections: BinaryHeap::new(),
            merged_intersections: Vec::new()}
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {pos_intersections: BinaryHeap::with_capacity(capacity),
            neg_intersections: BinaryHeap::with_capacity(capacity),
        merged_intersections: Vec::with_capacity(capacity)}
    }

    pub fn add_point(&mut self, intersection: Intersection<'a, T>) {
        self.merged_intersections.push(intersection.clone());
        if intersection.point > 0.0 {
            self.pos_intersections.push(Reverse(intersection));
        } else {
            self.neg_intersections.push(Reverse(intersection));
        }
    }

    pub fn hit(&self) -> Option<&Intersection<'a, T>> {
        match self.pos_intersections.peek() {
            Some(p) => Some(&p.0),
            None => None
        }
    }

    pub fn len(&self) -> usize {
        self.merged_intersections.len()
    }
}

impl<'a, T: PartialEq + Clone> Index<usize> for Intersections<'a, T> {
    type Output = Intersection<'a, T>;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.merged_intersections[idx]
    }
}

impl<'a, T: PartialEq + Clone> Ord for Intersection<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if utils::is_equal(self.point, other.point) {
            return Ordering::Equal;
        }
        if self.point < other.point {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

impl<'a, T: PartialEq + Clone> PartialOrd for Intersection<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if utils::is_equal(self.point, other.point) {
            return Some(Ordering::Equal);
        }
        if self.point < other.point {
            return Some(Ordering::Less);
        } else {
            return Some(Ordering::Greater);
        }
    }
}

impl<'a, T: PartialEq + Clone> PartialEq for Intersection<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && self.point == other.point
    }
}


impl<'a, T: PartialEq + Clone> Intersection<'a, T> {

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

impl<'a, T: PartialEq + Clone> Eq for Intersection<'a, T> {}

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

    #[test]
    fn test_hit_all_positive() {
        let s = Sphere::new(1);
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let xs = Intersections::new(vec![i1.clone(), i2]);
        let h = xs.hit().unwrap();
        assert_eq!(*h, i1);
    }

    #[test]
    fn test_hit_some_negative() {
        let s = Sphere::new(1);
        let i1 = Intersection::new(&s, -11.0);
        let i2 = Intersection::new(&s, 1.0);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);
        let h = xs.hit().unwrap();
        assert_eq!(*h, i2);
    }

    #[test]
    fn test_hit_all_negative() {
        let s = Sphere::new(1);
        let i1 = Intersection::new(&s, -2.0);
        let i2 = Intersection::new(&s, -1.0);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);
        let h = xs.hit();
        assert_eq!(h, None);
    }

    #[test]
    fn test_hit_multiple_pos_neg() {
        let s = Sphere::new(1);
        let i1 = Intersection::new(&s, 5.0);
        let i2 = Intersection::new(&s, 7.0);
        let i3 = Intersection::new(&s, -3.0);
        let i4 = Intersection::new(&s, 2.0);
        let xs = Intersections::new(vec![i3.clone(), i4.clone(), i1.clone(), i2.clone()]);
        let h = xs.hit().unwrap();
        assert_eq!(*h, i4);

    }

}
