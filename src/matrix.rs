use std::ops;
use crate::tuple::Tuple;

#[derive(Clone)]
pub struct Matrix{pub m: Vec<Vec<f64>>}
impl Matrix {
  pub fn identity() -> Matrix {
    Matrix{m: vec![
      vec![1.0, 0.0, 0.0, 0.0],
      vec![0.0, 1.0, 0.0, 0.0],
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0]
    ]}
  }
  pub fn equals(&self, m: Self) -> bool {
    const EPSILON: f64 = 0.00001;
    if self.m.len() != m.m.len() || self.m[0].len() != m.m[0].len(){
      return false;
    }
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
impl ops::Mul<Matrix> for Matrix {
  type Output = Self;
  fn mul(self, _rhs: Matrix) -> Self::Output {
    let a = self.m;
    let b = _rhs.m;
    let mut m = Matrix{m: vec![vec![0.0; 4]; 4]};
    for i in 0..4 {
      for j in 0..4 {
        m.m[i][j] = a[i][0] * b[0][j] +
                    a[i][1] * b[1][j] +
                    a[i][2] * b[2][j] +
                    a[i][3] * b[3][j];
      } 
    }
    m
  }
}
impl ops::Mul<Tuple> for Matrix {
  type Output = Tuple;
  fn mul(self, _rhs: Tuple) -> Self::Output {
    let a = self.m;
    let b = _rhs;
    Tuple::new(
      a[0][0] * b.x + a[0][1] * b.y + a[0][2] * b.z + a[0][3] * b.w,
      a[1][0] * b.x + a[1][1] * b.y + a[1][2] * b.z + a[1][3] * b.w,
      a[2][0] * b.x + a[2][1] * b.y + a[2][2] * b.z + a[2][3] * b.w,
      a[3][0] * b.x + a[3][1] * b.y + a[3][2] * b.z + a[3][3] * b.w,
    )
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
  let a = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.0, 6.0, 7.0, 8.0],
    vec![9.0, 8.0, 7.0, 6.0],
    vec![5.0, 4.0, 3.0, 2.0]
  ]};
  let b = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.0, 6.0, 7.0, 8.0],
    vec![9.0, 8.0, 7.0, 6.0],
    vec![5.0, 4.0, 3.0, 2.0]
  ]};
  assert!(a.equals(b));
}
#[test]
fn matrix_equality_with_different_matrices() {
  let a = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.0, 6.0, 7.0, 8.0],
    vec![9.0, 8.0, 7.0, 6.0],
    vec![5.0, 4.0, 3.0, 2.0]
  ]};
  let b = Matrix{m: vec![
    vec![2.0, 3.0, 4.0, 5.0],
    vec![6.0, 7.0, 8.0, 9.0],
    vec![8.0, 7.0, 6.0, 5.0],
    vec![4.0, 3.0, 2.0, 1.0]
  ]};
  assert!(!a.equals(b));
}
#[test]
fn multiplying_two_matrices() {
  let a = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![5.0, 6.0, 7.0, 8.0],
    vec![9.0, 8.0, 7.0, 6.0],
    vec![5.0, 4.0, 3.0, 2.0]
  ]};
  let b = Matrix{m: vec![
    vec![-2.0, 1.0, 2.0, 3.0],
    vec![3.0, 2.0, 1.0, -1.0],
    vec![4.0, 3.0, 6.0, 5.0],
    vec![1.0, 2.0, 7.0, 8.0]
  ]};
  let c = Matrix{m: vec![
    vec![20.0, 22.0, 50.0, 48.0],
    vec![44.0, 54.0, 114.0, 108.0],
    vec![40.0, 58.0, 110.0, 102.0],
    vec![16.0, 26.0, 46.0, 42.0]
  ]};
  assert!((a * b).equals(c));
}
#[test]
fn multiplying_a_matrix_by_a_tuple() {
  let a = Matrix{m: vec![
    vec![1.0, 2.0, 3.0, 4.0],
    vec![2.0, 4.0, 4.0, 2.0],
    vec![8.0, 6.0, 4.0, 1.0],
    vec![0.0, 0.0, 0.0, 1.0]
  ]};
  let t1 = Tuple::new(1.0, 2.0, 3.0, 1.0);
  let t2 = Tuple::new(18.0, 24.0, 33.0, 1.0);
  assert!((a * t1).equals(t2));
}
#[test]
fn multiplying_a_matrix_by_the_identity_matrix() {
  let a = Matrix{m: vec![
    vec![0.0, 1.0, 2.0, 4.0],
    vec![1.0, 2.0, 4.0, 8.0],
    vec![2.0, 4.0, 8.0, 16.0],
    vec![4.0, 8.0, 16.0, 32.0]
  ]};
  let b = a.clone();
  let identity = Matrix::identity();
  assert!((a * identity).equals(b));
}
#[test]
fn multiplying_a_tuple_by_the_identity_matrix() {
  let t = Tuple::new(1.0, 2.0, 3.0, 4.0);
  let identity = Matrix::identity();
  assert!((identity * t).equals(t));
}
