use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::sphere::Sphere;
use std::ptr;

pub trait Intersect {
  fn intersects(&self, r: Ray) -> Vec<Intersection<&Self>>;
}

#[derive(Copy, Clone)]
pub enum Shape {
  Sphere(Sphere)
}

pub fn hit(intersections: Vec<Intersection<Shape>>) -> Option<Intersection<Shape>>{
  let mut t_min = f64::MAX;
  let mut hit = intersections[0];
  for i in intersections {
    if i.t > 0.0 && i.t < t_min {
      t_min = i.t;
      hit = i.clone();
    }
  }
  if t_min != f64::MAX {
    Some(hit)
  }
  else {
    None
  }
}

#[derive(Copy, Clone)]
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

#[test]
fn hit_when_all_intersections_have_positive_t() {
  let s = Shape::Sphere(Sphere::new());
  let i1 = Intersection{t: 1.0, object: s};
  let i2 = Intersection{t: 2.0, object: s};
  let xs = vec!(i1, i2);
  let i = hit(xs);
  assert_eq!((i.unwrap()).t, i1.t);
}

#[test]
fn hit_when_some_intersections_have_negative_t() {
  let s = Shape::Sphere(Sphere::new());
  let i1 = Intersection{t: -1.0, object: s};
  let i2 = Intersection{t: 1.0, object: s};
  let xs = vec!(i1, i2);
  let i = hit(xs);
  assert_eq!((i.unwrap()).t, i2.t);
}

#[test]
fn hit_when_all_intersections_have_negative_t() {
  let s = Shape::Sphere(Sphere::new());
  let i1 = Intersection{t: -2.0, object: s};
  let i2 = Intersection{t: -1.0, object: s};
  let xs = vec!(i1, i2);
  let i = hit(xs);
  assert!(i.is_none());
}

#[test]
fn the_hit_is_always_the_lowest_nonnegative_intersection() {
  let s = Shape::Sphere(Sphere::new());
  let i1 = Intersection{t: 5.0, object: s};
  let i2 = Intersection{t: 7.0, object: s};
  let i3 = Intersection{t: -3.0, object: s};
  let i4 = Intersection{t: 2.0, object: s};
  let xs = vec!(i1, i2, i3, i4);
  let i = hit(xs);
  assert_eq!((i.unwrap()).t, i4.t);
}