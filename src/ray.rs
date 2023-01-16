use crate::tuple::Tuple;
use crate::matrix::Matrix;

pub struct Ray{ pub origin: Tuple, pub direction: Tuple }
impl Ray{
  pub fn new(origin: Tuple, direction: Tuple) -> Ray {
    Ray{origin: origin, direction: direction}
  }
  pub fn position(&self, t: f64) -> Tuple {
    self.origin + self.direction * t
  }
} 

#[test]
fn creating_and_querying_an_array() {
  let origin = Tuple::point(1.0, 2.0, 3.0);
  let direction = Tuple::vector(4.0, 5.0, 6.0);
  let r = Ray::new(origin, direction);
  assert!(r.origin.equals(origin));
  assert!(r.direction.equals(direction));
}
#[test]
fn computing_a_point_from_a_distance() {
  let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
  assert!(r.position(0.0).equals(Tuple::point(2.0, 3.0, 4.0)));
  assert!(r.position(1.0).equals(Tuple::point(3.0, 3.0, 4.0)));
  assert!(r.position(-1.0).equals(Tuple::point(1.0, 3.0, 4.0)));
  assert!(r.position(2.5).equals(Tuple::point(4.5, 3.0, 4.0)));
}
#[test]
fn translating_a_ray() {
  let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
  let m = Matrix::translation(3.0, 4.0, 5.0);
  let r2 = m * r;
  assert!(r2.origin.equals(Tuple::point(4.0, 6.0, 8.0)));
  assert!(r2.direction.equals(Tuple::vector(0.0, 1.0, 0.0)));

}
#[test]
fn scaling_a_ray() {
  let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
  let m = Matrix::scaling(2.0, 3.0, 4.0);
  let r2 = m * r;
  assert!(r2.origin.equals(Tuple::point(2.0, 6.0, 12.0)));
  assert!(r2.direction.equals(Tuple::vector(0.0, 3.0, 0.0)));
}
