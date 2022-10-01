use std::f64::consts::PI;
use crate::tuple::Tuple;
use crate::matrix::Matrix;

#[test]
fn multiplying_by_a_translation_matrix() {
  let t = Matrix::translation(5.0, -3.0, 2.0);
  let p = Tuple::point(-3.0, 4.0, 5.0);
  assert!((t * p).equals(Tuple::point(2.0, 1.0, 7.0)));
}
#[test]
fn multiplying_by_the_inverse_of_a_translation_matrix() {
  let t = Matrix::translation(5.0, -3.0, 2.0);
  let i = t.inverse();
  let p = Tuple::point(-3.0, 4.0, 5.0);
  assert!((i * p).equals(Tuple::point(-8.0, 7.0, 3.0)));
}
#[test]
fn translation_does_not_affect_vectors() {
  let t = Matrix::translation(5.0, -3.0, 2.0);
  let v = Tuple::vector(-3.0, 4.0, 5.0);
  assert!((t * v).equals(v));
}
#[test]
fn a_scaling_matrix_applied_to_a_point() {
  let t = Matrix::scaling(2.0, 3.0, 4.0);
  let p = Tuple::point(-4.0, 6.0, 8.0);
  assert!((t * p).equals(Tuple::point(-8.0, 18.0, 32.0)));
}
#[test]
fn a_scaling_matrix_applied_to_a_vector() {
  let t = Matrix::scaling(2.0, 3.0, 4.0);
  let v = Tuple::vector(-4.0, 6.0, 8.0);
  assert!((t * v).equals(Tuple::vector(-8.0, 18.0, 32.0)));
}
#[test]
fn multiplying_by_the_inverse_of_a_scaling_matrix() {
  let t = Matrix::scaling(2.0, 3.0, 4.0);
  let i = t.inverse();
  let v = Tuple::vector(-4.0, 6.0, 8.0);
  assert!((i * v).equals(Tuple::vector(-2.0, 2.0, 2.0)));
}
#[test]
fn reflection_is_scaling_by_a_negative_value() {
  let t = Matrix::scaling(-1.0, 1.0, 1.0);
  let p = Tuple::point(2.0, 3.0, 4.0);
  assert!((t * p).equals(Tuple::point(-2.0, 3.0, 4.0)));
}
#[test]
fn rotating_a_point_around_the_x_axis() {
  let p = Tuple::point(0.0, 1.0, 0.0);
  let hq = Matrix::rotation_x(PI/4.0);
  let fq = Matrix::rotation_x(PI/2.0);
  assert!((hq * p).equals(Tuple::point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)));
  assert!((fq * p).equals(Tuple::point(0.0, 0.0, 1.0)));
}
#[test]
fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
  let p = Tuple::point(0.0, 1.0, 0.0);
  let hq = Matrix::rotation_x(PI/4.0);
  let inv = hq.inverse();
  assert!((inv * p).equals(Tuple::point(0.0, f64::sqrt(2.0) / 2.0, -(f64::sqrt(2.0) / 2.0))));
}
#[test]
fn rotating_a_point_around_the_y_axis() {
  let p = Tuple::point(0.0, 0.0, 1.0);
  let hq = Matrix::rotation_y(PI/4.0);
  let fq = Matrix::rotation_y(PI/2.0);
  assert!((hq * p).equals(Tuple::point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)));
  assert!((fq * p).equals(Tuple::point(1.0, 0.0, 0.0)));
}
#[test]
fn rotating_a_point_around_the_z_axis() {
  let p = Tuple::point(0.0, 1.0, 0.0);
  let hq = Matrix::rotation_z(PI/4.0);
  let fq = Matrix::rotation_z(PI/2.0);
  assert!((hq * p).equals(Tuple::point(-(f64::sqrt(2.0) / 2.0), f64::sqrt(2.0) / 2.0, 0.0)));
  assert!((fq * p).equals(Tuple::point(-1.0, 0.0, 0.0)));
}
