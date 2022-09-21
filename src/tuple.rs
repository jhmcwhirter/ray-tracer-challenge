use std::ops;

#[derive(Copy, Clone)]
pub struct Tuple{ pub x: f64, pub y: f64, pub z: f64, pub w: f64 }

impl Tuple {
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    Tuple{x: x, y: y, z: z, w: w}
  }
  pub fn is_point(&self) -> bool {
    self.w == 1.0
  }
  pub fn is_vector(&self) -> bool {
    self.w == 0.0
  }
  pub fn equals(&self, t: Self) -> bool {
    const EPSILON: f64 = 0.00001;
    (self.x - t.x).abs() < EPSILON && (self.y - t.y).abs() < EPSILON && (self.z - t.z).abs() < EPSILON && (self.w - t.w).abs() < EPSILON
  }
  pub fn magnitude(&self) -> f64 {
    (f64::powf(self.x, 2.0) + f64::powf(self.y, 2.0) + f64::powf(self.z, 2.0) + f64::powf(self.w, 2.0)).sqrt()
  }
  pub fn normalize(&self) -> Self {
    let m = self.magnitude();
    Tuple::new(self.x / m, self.y / m, self.z / m, self.w / m) 
  }
  pub fn dot(&self, t: Self) -> f64 {
    self.x * t.x + self.y * t.y + self.z * t.z + self.w * t.w
  }
  pub fn cross(&self, t: Self) -> Self {
    Tuple::new(
      self.y * t.z - self.z * t.y,
      self.z * t.x - self.x * t.z,
      self.x * t.y - self.y * t.x,
      0.0 // vector
    )
  }
  pub fn red(&self) -> f64 {
    self.x
  }
  pub fn green(&self) -> f64 {
    self.y
  }
  pub fn blue(&self) -> f64 {
    self.z
  }
  // constructors
  pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
  }
  pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
  }
  pub fn color(red: f64, green: f64, blue: f64) -> Tuple {
    Tuple::new(red, green, blue, 0.0) // colors are also vectors, this might be a problem later
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
impl ops::Mul<f64> for Tuple {
  type Output = Self;
  fn mul(self, _rhs: f64) -> Self::Output {
    Tuple::new(self.x * _rhs, self.y * _rhs, self.z * _rhs, self.w * _rhs)
  }
}
impl ops::Div<f64> for Tuple {
  type Output = Self;
  fn div(self, _rhs: f64) -> Self::Output {
    Tuple::new(self.x / _rhs, self.y / _rhs, self.z / _rhs, self.w / _rhs)
  }
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
fn point_fn_creates_points() {
  let p = Tuple::point(1.0, 2.0, 3.0);
  assert!(p.is_point())
}
#[test]
fn equal_points_are_equal() {
  let p1 = Tuple::point(1.0, 2.0, 3.0);
  let p2 = Tuple::point(1.0, 2.0, 3.0);
  assert!(p1.equals(p2));
}
#[test]
fn vector_fn_creates_vectors() {
  let v = Tuple::vector(1.0, 2.0, 3.0);
  assert!(v.is_vector())
}
#[test]
fn equal_vectors_are_equal() {
  let v1 = Tuple::vector(1.0, 2.0, 3.0);
  let v2 = Tuple::vector(1.0, 2.0, 3.0);
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
  let p1 = Tuple::point(3.0, 2.0, 1.0);
  let p2 = Tuple::point(5.0, 6.0, 7.0);
  assert!((p1 - p2).equals(Tuple::vector(-2.0, -4.0, -6.0)));
}
#[test]
fn subtracting_a_vector_from_a_point() {
  let p = Tuple::point(3.0, 2.0, 1.0);
  let v = Tuple::vector(5.0, 6.0, 7.0);
  assert!((p - v).equals(Tuple::point(-2.0, -4.0, -6.0)));
}
#[test]
fn subtracting_two_vectors() {
  let v1 = Tuple::vector(3.0, 2.0, 1.0);
  let v2 = Tuple::vector(5.0, 6.0, 7.0);
  assert!((v1 - v2).equals(Tuple::vector(-2.0, -4.0, -6.0)));
}
#[test]
fn subtracting_a_vector_from_the_zero_vector() {
  let zero = Tuple::vector(0.0, 0.0, 0.0);
  let v = Tuple::vector(1.0, -2.0, 3.0);
  assert!((zero - v).equals(Tuple::vector(-1.0, 2.0, -3.0)));
}
#[test]
fn negating_a_tuple() {
  let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert!((-t).equals(Tuple::new(-1.0, 2.0, -3.0, 4.0)));
}
#[test]
fn multiplying_a_tuple_by_a_scalar() {
  let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert!((t * 3.5).equals(Tuple::new(3.5, -7.0, 10.5, -14.0)));
}
#[test]
fn multiplying_a_tuple_by_a_fraction() {
  let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert!((t * 0.5).equals(Tuple::new(0.5, -1.0, 1.5, -2.0)));
}
#[test]
fn divding_a_tuple_by_a_scalar() {
  let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
  assert!((t / 2.0).equals(Tuple::new(0.5, -1.0, 1.5, -2.0)));
}
#[test]
fn magnitude_of_vectors() {
  let v1 = Tuple::vector(1.0, 0.0, 0.0);
  let v2 = Tuple::vector(0.0, 1.0, 0.0);
  let v3 = Tuple::vector(0.0, 0.0, 1.0);
  let v4 = Tuple::vector(1.0, 2.0, 3.0);
  let v5 = Tuple::vector(-1.0, -2.0, -3.0);
  assert_eq!(v1.magnitude(), 1.0);
  assert_eq!(v2.magnitude(), 1.0);
  assert_eq!(v3.magnitude(), 1.0);
  assert_eq!(v4.magnitude(), (14.0_f64).sqrt());
  assert_eq!(v5.magnitude(), (14.0_f64).sqrt());
}
#[test]
fn normalizing_vectors() {
  let v1 = Tuple::vector(4.0, 0.0, 0.0);
  let v2 = Tuple::vector(1.0, 2.0, 3.0);
  assert!(v1.normalize().equals(Tuple::vector(1.0, 0.0, 0.0)));
  assert!(v2.normalize().equals(Tuple::vector(0.26726, 0.53452, 0.80178)));
}
#[test]
fn dot_product_of_tuples() {
  let v1 = Tuple::vector(1.0, 2.0, 3.0);
  let v2 = Tuple::vector(2.0, 3.0, 4.0);
  assert_eq!(v1.dot(v2), 20.0);
}
#[test]
fn cross_product_of_vectors() {
  let v1 = Tuple::vector(1.0, 2.0, 3.0);
  let v2 = Tuple::vector(2.0, 3.0, 4.0);
  assert!(v1.cross(v2).equals(Tuple::vector(-1.0, 2.0, -1.0)));
  let v2 = Tuple::vector(2.0, 3.0, 4.0);
  assert!(v2.cross(v1).equals(Tuple::vector(1.0, -2.0, 1.0)));
}
#[test]
fn colors_are_tuples() {
  let c = Tuple::color(-0.5, 0.4, 1.7);
  assert_eq!(c.red(), -0.5);
  assert_eq!(c.green(), 0.4);
  assert_eq!(c.blue(), 1.7);
}
#[test]
fn adding_colors() {
  let c1 = Tuple::color(0.9, 0.6, 0.75);
  let c2 = Tuple::color(0.7, 0.1, 0.25);
  assert!((c1 + c2).equals(Tuple::color(1.6, 0.7, 1.0)));
}
#[test]
fn subtracting_colors() {
  let c1 = Tuple::color(0.9, 0.6, 0.75);
  let c2 = Tuple::color(0.7, 0.1, 0.25);
  assert!((c1 - c2).equals(Tuple::color(0.2, 0.5, 0.5)));
}
#[test]
fn multipyling_a_color_by_a_scalar() {
  let c = Tuple::color(0.2, 0.3, 0.4);
  assert!((c * 2.0).equals(Tuple::color(0.4, 0.6, 0.8)));
}
