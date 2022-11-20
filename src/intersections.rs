use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::sphere::Sphere;
use std::ptr;

pub trait Intersect {
  fn intersects(&self, r: Ray) -> Vec<Intersection<&Self>>;
}

pub enum Shape {
  Sphere
}

#[derive(Clone)]
pub struct Intersection<Shape> { pub t: f64, pub object: Shape }

#[test]
fn an_intersection_encapsulates_t_and_object() {
  let s = Sphere{};
  let i = Intersection{t: 3.5, object: &s};
  assert_eq!(i.t, 3.5);
  assert!(ptr::eq(i.object, &s));
}
#[test]
fn aggragating_intersections() {
  let s = Sphere{};
  let i1 = Intersection{t: 1.0, object: s};
  let i2 = Intersection{t: 2.0, object: s};
  let xs = [i1, i2];
  assert_eq!(xs.len(), 2);
  assert_eq!(xs[0].t, 1.0);
  assert_eq!(xs[1].t, 2.0);
}

#[test]
fn intersect_sets_the_object_on_the_intersection() {
  let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
  let s = Sphere::new();
  let xs = s.intersects(r);
  assert_eq!(xs.len(), 2);
  assert!(ptr::eq(xs[0].object, &s));
  assert!(ptr::eq(xs[1].object, &s));
}
