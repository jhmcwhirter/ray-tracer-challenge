//use std::ops;

#[derive(Clone)]
pub struct Matrix{pub m: Vec<Vec<f64>>}
impl Matrix {
  pub fn equals(&self, m: Self) -> bool {
    const EPSILON: f64 = 0.00001;
    for (i, row) in self.m.iter().enumerate() {
      for (j, col) in row.iter().enumerate() {
        if col - m.m[i][j] >= EPSILON {
          return false;
        }
      }
    } 
    return true;
  }
}

#[test]
fn constructing_and_inspecting_a_4x4_matrix() {
  let m = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.5, 6.5, 7.5, 8.5],
    vec![9.0, 10.0, 11.0, 12.0],
    vec![13.5, 14.5, 15.5, 16.5]
  ]};
  assert_eq!(m.m[0][0], 1.0);
  assert_eq!(m.m[0][3], 4.0);
  assert_eq!(m.m[1][0], 5.5);
  assert_eq!(m.m[1][2], 7.5);
  assert_eq!(m.m[2][2], 11.0);
  assert_eq!(m.m[3][0], 13.5);
  assert_eq!(m.m[3][2], 15.5);
}
#[test]
fn a_2x2_matrix_ought_to_be_representable() {
  let m = Matrix{m: vec![
    vec![-3.0, 5.0],
    vec![1.0, -2.0]
  ]};
  assert_eq!(m.m[0][0], -3.0);
  assert_eq!(m.m[0][1], 5.0);
  assert_eq!(m.m[1][0], 1.0);
  assert_eq!(m.m[1][1], -2.0);
}
#[test]
fn a_3x3_matrix_ought_to_be_representable() {
  let m = Matrix{m: vec![
    vec![-3.0, 5.0, 0.0],
    vec![1.0, -2.0, -7.0],
    vec![0.0, 1.0, 1.0]
  ]};
  assert_eq!(m.m[0][0], -3.0);
  assert_eq!(m.m[1][1], -2.0);
  assert_eq!(m.m[2][2], 1.0);
}
#[test]
fn matrix_equality_with_identical_matrices() {
  let m = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.0, 6.0, 7.0, 8.0],
    vec![9.0, 8.0, 7.0, 6.0],
    vec![5.0, 4.0, 3.0, 2.0]
  ]};
  let p = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.0, 6.0, 7.0, 8.0],
    vec![9.0, 8.0, 7.0, 6.0],
    vec![5.0, 4.0, 3.0, 2.0]
  ]};
  assert!(m.equals(p));
}
#[test]
fn matrix_equality_with_different_matrices() {
  let m = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.0, 6.0, 7.0, 8.0],
    vec![9.0, 8.0, 7.0, 6.0],
    vec![5.0, 4.0, 3.0, 2.0]
  ]};
  let p = Matrix{m: vec![
    vec![2.0, 3.0, 4.0, 5.0],
    vec![6.0, 7.0, 8.0, 9.0],
    vec![8.0, 7.0, 6.0, 5.0],
    vec![4.0, 3.0, 2.0, 1.0]
  ]};
  assert!(!m.equals(p));
}