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
