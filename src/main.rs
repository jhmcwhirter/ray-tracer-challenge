use std::ops;

struct Tuple{ pub x: f64, pub y: f64, pub z: f64, pub w: f64 }
impl Tuple {
  
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple{x: x, y: y, z: z, w: w}
  }
  pub fn is_point(&self) -> bool {
    self.w == 1.0
  }
  pub fn is_vector(&self) -> bool {
    self.w == 0.0
  }
  pub fn equals(&self, t: Tuple) -> bool {
    const EPSILON: f64 = 0.00001;
    (self.x - t.x).abs() < EPSILON && (self.y - t.y).abs() < EPSILON && (self.z - t.z).abs() < EPSILON && (self.w - t.w).abs() < EPSILON
  }
  pub fn minus(&self, t: Tuple) -> Tuple {
    Tuple::new(self.x - t.x, self.y - t.y, self.z - t.z, self.w - t.w)
  }
}
impl ops::Add for Tuple {
  type Output = Self;
  fn add(self, _rhs: Self) -> Self::Output {
    Tuple::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z, self.w + _rhs.w)
  }
}
impl ops::Sub for Tuple {
  type Output = Self;
  fn sub(self, _rhs: Self) -> Self::Output {
    Tuple::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z, self.w - _rhs.w)
  }
}
impl ops::Neg for Tuple {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Tuple::new(-self.x, -self.y, -self.z, -self.w)
  }
}

fn point(x: f64, y: f64, z: f64) -> Tuple {
  Tuple::new(x, y, z, 1.0)
}
fn vector(x: f64, y: f64, z: f64) -> Tuple {
  Tuple::new(x, y, z, 0.0)
}

#[test]
fn tuple_with_w_eq_1_is_a_point() {
  let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
  assert_eq!(t.x, 1.0);
  assert_eq!(t.y, 2.0);
  assert_eq!(t.z, 3.0);
  assert_eq!(t.w, 1.0);
  assert!(t.is_point());
  assert!(!t.is_vector());
}
#[test]
fn tuple_with_w_eq_0_is_a_vector() {
  let t = Tuple::new(1.0, 2.0, 3.0, 0.0);
  assert_eq!(t.x, 1.0);
  assert_eq!(t.y, 2.0);
  assert_eq!(t.z, 3.0);
  assert_eq!(t.w, 0.0);
  assert!(t.is_vector());
  assert!(!t.is_point());
}
#[test]
fn equal_tuples_are_equal() {
  let t1 = Tuple::new(1.0, 2.0, 3.0, 0.0);
  let t2 = Tuple::new(1.0, 2.0, 3.0, 0.0);
  assert!(t1.equals(t2));
}
#[test]
fn point_creates_points() {
  let p = point(1.0, 2.0, 3.0);
  assert!(p.is_point())
}
#[test]
fn equal_points_are_equal() {
  let p1 = point(1.0, 2.0, 3.0);
  let p2 = point(1.0, 2.0, 3.0);
  assert!(p1.equals(p2));
}
#[test]
fn vector_creates_vectors() {
  let v = vector(1.0, 2.0, 3.0);
  assert!(v.is_vector())
}
#[test]
fn equal_vectors_are_equal() {
  let v1 = vector(1.0, 2.0, 3.0);
  let v2 = vector(1.0, 2.0, 3.0);
  assert!(v1.equals(v2));
}
#[test]
fn adding_two_tuples() {
  let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
  let t2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
  assert!((t1 + t2).equals(Tuple::new(1.0, 1.0, 6.0, 1.0)));
}
#[test]
fn subtracting_two_points() {
  let p1 = point(3.0, 2.0, 1.0);
  let p2 = point(5.0, 6.0, 7.0);
  assert!((p1 - p2).equals(vector(-2.0, -4.0, -6.0)));
}
#[test]
fn subtracting_a_vector_from_a_point() {
  let p = point(3.0, 2.0, 1.0);
  let v = vector(5.0, 6.0, 7.0);
  assert!((p - v).equals(point(-2.0, -4.0, -6.0)));
}
#[test]
fn subtracting_two_vectors() {
  let v1 = vector(3.0, 2.0, 1.0);
  let v2 = vector(5.0, 6.0, 7.0);
  assert!((v1 - v2).equals(vector(-2.0, -4.0, -6.0)));
}
#[test]
fn subtracting_a_vector_from_the_zero_vector() {
  let zero = vector(0.0, 0.0, 0.0);
  let v = vector(1.0, -2.0, 3.0);
  assert!((zero - v).equals(vector(-1.0, 2.0, -3.0)));
}
#[test]
fn negating_a_tuple() {
  let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert!((-t).equals(Tuple::new(-1.0, 2.0, -3.0, 4.0)));
}

fn main() {
  
}

