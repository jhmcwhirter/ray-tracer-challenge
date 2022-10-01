use std::ops;
use crate::tuple::Tuple;

#[derive(Clone)]
pub struct Matrix{pub m: Vec<Vec<f64>>}
impl Matrix {
  pub fn identity() -> Self {
    Matrix{m: vec![
      vec![1.0, 0.0, 0.0, 0.0],
      vec![0.0, 1.0, 0.0, 0.0],
      vec![0.0, 0.0, 1.0, 0.0],
      vec![0.0, 0.0, 0.0, 1.0]
    ]}
  }
  pub fn equals(&self, m: Self) -> bool {
    const EPSILON: f64 = 0.00001;
    if self.rows() != m.rows() || self.cols() != m.cols() {
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
  pub fn rows(&self) -> usize {
    self.m.len()
  }
  pub fn cols(&self) -> usize {
    self.m[0].len()
  }
  pub fn transpose(&self) -> Self {
    let mut m = Matrix{m: vec![vec![0.0; 4]; 4]};
    for i in 0..4 {
      for j in 0..4 {
        m.m[i][i] = self.m[j][i];
      }
    }
    m
  }
  pub fn determinant(&self) -> f64 {
    if self.rows() == 2 {
      let a = self.m[0][0];
      let b = self.m[0][1];
      let c = self.m[1][0];
      let d = self.m[1][1];
      a * d - b * c
    }
    else {
      let mut det = 0.0;
      for col in 0..self.cols() {
        det += self.m[0][col] * self.cofactor(0, col);
      }
      det
    }
  }
  pub fn submatrix(&self, row: usize, col: usize) -> Self {
    let mut m = Matrix{m: vec![]};
    for r in 0..self.rows() {
      let mut m_row = vec![]; 
      if r == row {
        continue;
      } 
      for c in 0..self.cols() {
        if c == col {
          continue;
        }
        m_row.push(self.m[r][c]);
      }
      m.m.push(m_row);
    }
    m
  }
  pub fn minor(&self, row: usize, col: usize) -> f64 {
    self.submatrix(row, col).determinant()
  }
  pub fn cofactor(&self, row: usize, col: usize) -> f64 {
    if (row + col) % 2 == 0 {
      self.minor(row, col)
    } 
    else {
      self.minor(row, col) * -1.0
    }
  }
  pub fn invertible(&self) -> bool {
    !(self.determinant() == 0.0)
  }
  pub fn inverse(&self) -> Self {
    if !self.invertible() {
      panic!("trying to invert a non-invertible matrix");
    }
    let mut m = Matrix{m: vec![vec![0.0; self.cols()]; self.rows()]};
    for row in 0..self.rows() {
      for col in 0..self.cols() {
        let c = self.cofactor(row, col);
        m.m[col][row] = c / self.determinant();
      }
    }
    m
  }
  pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity();
    m.m[0][3] = x;
    m.m[1][3] = y;
    m.m[2][3] = z;
    m
  }
}
impl ops::Mul<Self> for Matrix {
  type Output = Self;
  fn mul(self, _rhs: Self) -> Self::Output {
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
#[test]
fn transposing_a_matrix() {
  let a = Matrix{m: vec![
    vec![0.0, 9.0, 3.0, 0.0],
    vec![9.0, 8.0, 0.0, 8.0],
    vec![1.0, 8.0, 5.0, 3.0],
    vec![0.0, 0.0, 5.0, 8.0]
  ]};
  let b = Matrix{m: vec![
    vec![0.0, 9.0, 1.0, 0.0],
    vec![9.0, 8.0, 8.0, 0.0],
    vec![3.0, 0.0, 5.0, 5.0],
    vec![0.0, 8.0, 3.0, 8.0]
  ]};
  assert!(a.transpose().equals(b));
}
#[test]
fn transposing_the_identity_matrix() {
  let identity = Matrix::identity();
  let a = identity.transpose();
  assert!(a.equals(identity));
}
#[test]
fn calculating_the_determinant_of_a_2x2_matrix() {
  let a = Matrix{m: vec![
    vec![1.0, 5.0],
    vec![-3.0, 2.0]
  ]};
  assert_eq!(a.determinant(), 17.0);
}
#[test]
fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
  let a = Matrix{m: vec![
    vec![1.0, 5.0, 0.0],
    vec![-3.0, 2.0, 7.0],
    vec![0.0, 6.0, -3.0]
  ]};
  let b = Matrix{m: vec![
    vec![-3.0, 2.0],
    vec![0.0, 6.0]
  ]};
  assert!(a.submatrix(0, 2).equals(b));
}
#[test]
fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
  let a = Matrix{m: vec![
    vec![-6.0, 1.0, 1.0, 6.0],
    vec![-8.0, 5.0, 8.0, 6.0],
    vec![-1.0, 0.0, 8.0, 2.0],
    vec![-7.0, 1.0, -1.0, 1.0]
  ]};
  let b = Matrix{m: vec![
    vec![-6.0, 1.0, 6.0],
    vec![-8.0, 8.0, 6.0],
    vec![-7.0, -1.0, 1.0]
  ]};
  assert!(a.submatrix(2, 1).equals(b));
}
#[test]
fn calculating_a_minor_of_a_3x3_matrix() {
  let a = Matrix{m: vec![
    vec![3.0, 5.0, 0.0],
    vec![2.0, -1.0, -7.0],
    vec![6.0, -1.0, 5.0]
  ]};
  let b = a.submatrix(1, 0);
  assert_eq!(b.determinant(), 25.0);
  assert_eq!(a.minor(1, 0), 25.0);
}
#[test]
fn calculating_a_cofactor_of_a_3x3_matrix() {
  let a = Matrix{m: vec![
    vec![3.0, 5.0, 0.0],
    vec![2.0, -1.0, -7.0],
    vec![6.0, -1.0, 5.0]
  ]};
  assert_eq!(a.minor(0, 0), -12.0);
  assert_eq!(a.cofactor(0, 0), -12.0);
  assert_eq!(a.minor(1, 0), 25.0);
  assert_eq!(a.cofactor(1, 0), -25.0);
}
#[test]
fn calculating_the_determinant_of_a_3x3_matrix() {
  let a = Matrix{m: vec![
    vec![1.0, 2.0, 6.0],
    vec![-5.0, 8.0, -4.0],
    vec![2.0, 6.0, 4.0]
  ]};
  assert_eq!(a.cofactor(0, 0), 56.0);
  assert_eq!(a.cofactor(0, 1), 12.0);
  assert_eq!(a.cofactor(0, 2), -46.0);
  assert_eq!(a.determinant(), -196.0);
}
#[test]
fn calculating_the_determinant_of_a_4x4_matrix() {
  let a = Matrix{m: vec![
    vec![-2.0, -8.0, 3.0, 5.0],
    vec![-3.0, 1.0, 7.0, 3.0],
    vec![1.0, 2.0, -9.0, 6.0],
    vec![-6.0, 7.0, 7.0, -9.0]
  ]};
  assert_eq!(a.cofactor(0, 0), 690.0);
  assert_eq!(a.cofactor(0, 1), 447.0);
  assert_eq!(a.cofactor(0, 2), 210.0);
  assert_eq!(a.cofactor(0, 3), 51.0);
  assert_eq!(a.determinant(), -4071.0);
}
#[test]
fn testing_an_invertible_matrix_for_invertability() {
  let a = Matrix{m: vec![
    vec![6.0, 4.0, 4.0, 4.0],
    vec![5.0, 5.0, 7.0, 6.0],
    vec![4.0, -9.0, 3.0, -7.0],
    vec![9.0, 1.0, 7.0, -6.0]
  ]};
  assert_eq!(a.determinant(), -2120.0);
  assert!(a.invertible());
}
#[test]
fn testing_a_noninvertible_matrix_for_invertability() {
  let a = Matrix{m: vec![
    vec![-4.0, 2.0, -2.0, -3.0],
    vec![9.0, 6.0, 2.0, 6.0],
    vec![0.0, -5.0, 1.0, -5.0],
    vec![0.0, 0.0, 0.0, 0.0]
  ]};
  assert_eq!(a.determinant(), 0.0);
  assert!(!a.invertible());
}
#[test]
fn calculating_the_inverse_of_a_matrix() {
  let a = Matrix{m: vec![
    vec![-5.0, 2.0, 6.0, -8.0],
    vec![1.0, -5.0, 1.0, 8.0],
    vec![7.0, 7.0, -6.0, -7.0],
    vec![1.0, -3.0, 7.0, 4.0]
  ]};
  let b = a.inverse();
  assert_eq!(a.determinant(), 532.0);
  assert_eq!(a.cofactor(2, 3), -160.0);
  assert_eq!(b.m[3][2], -160.0/532.0);
  assert_eq!(a.cofactor(3, 2), 105.0);
  assert_eq!(b.m[2][3], 105.0/532.0);
  assert!(b.equals(Matrix{m: vec![
    vec![0.21805, 0.45113, 0.24060, -0.04511],
    vec![-0.80827, -1.45677, -0.44361, 0.52068],
    vec![-0.07895, -0.22368, -0.05263, 0.19737],
    vec![-0.52256, -0.81391, -0.30075, 0.30639]
  ]}));
}
#[test]
fn calculating_the_inverse_of_another_matrix() {
  let a = Matrix{m: vec![
    vec![8.0, -5.0, 9.0, 2.0],
    vec![7.0, 5.0, 6.0, 1.0],
    vec![-6.0, 0.0, 9.0, 6.0],
    vec![-3.0, 0.0, -9.0, -4.0]
  ]};
  let b = a.inverse();
  assert!(b.equals(Matrix{m: vec![
    vec![-0.15385, -0.15385, -0.28205, -0.53846],
    vec![-0.07692, 0.12308, 0.02564, 0.03077],
    vec![0.35897, 0.35897, 0.43590, 0.92308],
    vec![-0.69231, -0.69231, -0.76923, -1.92308]
  ]}));
}
#[test]
fn calculating_the_inverse_of_a_third_matrix() {
  let a = Matrix{m: vec![
    vec![9.0, 3.0, 0.0, 9.0],
    vec![-5.0, -2.0, -6.0, -3.0],
    vec![-4.0, 9.0, 6.0, 4.0],
    vec![-7.0, 6.0, 6.0, 2.0]
  ]};
  let b = a.inverse();
  assert!(b.equals(Matrix{m: vec![
    vec![-0.04074, -0.07778, 0.14444, -0.22222],
    vec![-0.07778, 0.03333, 0.36667, -0.33333],
    vec![-0.02901, -0.14630, -0.10926, 0.12963],
    vec![0.17778, 0.06667, -0.26667, 0.33333]
  ]}));
}
#[test]
fn multiplying_a_product_by_its_inverse() {
  let a = Matrix{m: vec![
    vec![3.0, -9.0, 7.0, 3.0],
    vec![3.0, -8.0, 2.0, -9.0],
    vec![-4.0, 4.0, 4.0, 1.0],
    vec![-6.0, 5.0, -1.0, 1.0]
  ]};
  let b = Matrix{m: vec![
    vec![8.0, 2.0, 2.0, 2.0],
    vec![3.0, -1.0, 7.0, 0.0],
    vec![7.0, 0.0, 5.0, 4.0],
    vec![6.0, -2.0, 0.0, 5.0]
  ]};
  let b_clone = b.clone();
  let a_clone = a.clone();
  let c = a * b_clone;
  assert!((c * b.inverse()).equals(a_clone));
}
