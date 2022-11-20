use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::intersections::Intersect;
use crate::intersections::Intersection;

#[derive(Copy, Clone)]
pub struct Sphere{}
impl Sphere {
  pub fn new() -> Sphere {
    Sphere{}
  }
}
impl Intersect for Sphere {
  fn intersects(&self, r: Ray) -> Vec<Intersection<&Sphere>> {
    let sphere_to_ray = r.origin - Tuple::point(0.0, 0.0, 0.0);
    let a = r.direction.dot(r.direction);
    let b = 2.0 * r.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
    let discriminant = f64::powf(b, 2.0) - 4.0 * a * c;
    if discriminant < 0.0 {
      vec!()
    }
    else {
     let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
     let t2 = (-b + discriminant.sqrt()) / (2.0 * a); 
     vec![Intersection{t: t1, object: &self}, Intersection{t: t2, object: &self}]
    }
    
  }
}

#[test]
fn a_ray_intersects_a_sphere_at_2_points() {
  let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
  let s = Sphere::new();
  let xs = s.intersects(r);
  assert_eq!(xs.len(), 2);
  assert_eq!(xs[0].t, 4.0);
  assert_eq!(xs[1].t, 6.0);
}
#[test]
fn a_ray_intersects_a_sphere_at_a_tangent() {
  let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
  let s = Sphere::new();
  let xs = s.intersects(r);
  assert_eq!(xs.len(), 2);
  assert_eq!(xs[0].t, 5.0);
  assert_eq!(xs[1].t, 5.0);
}
#[test]
fn a_ray_misses_a_sphere() {
  let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
  let s = Sphere::new();
  let xs = s.intersects(r);
  assert_eq!(xs.len(), 0);
}
#[test]
fn a_ray_originates_inside_a_sphere() {
  let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
  let s = Sphere::new();
  let xs = s.intersects(r);
  assert_eq!(xs.len(), 2);
  assert_eq!(xs[0].t, -1.0);
  assert_eq!(xs[1].t, 1.0);
}
#[test]
fn a_ray_originates_behind_a_sphere() {
  let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
  let s = Sphere::new();
  let xs = s.intersects(r);
  assert_eq!(xs.len(), 2);
  assert_eq!(xs[0].t, -6.0);
  assert_eq!(xs[1].t, -4.0);
}
