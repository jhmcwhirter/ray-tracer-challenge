use std::ops;

#[derive(Copy, Clone)]
struct Tuple{ pub x: f64, pub y: f64, pub z: f64, pub w: f64 }
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
fn point_fn_creates_points() {
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
fn vector_fn_creates_vectors() {
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
  let v1 = vector(1.0, 0.0, 0.0);
  let v2 = vector(0.0, 1.0, 0.0);
  let v3 = vector(0.0, 0.0, 1.0);
  let v4 = vector(1.0, 2.0, 3.0);
  let v5 = vector(-1.0, -2.0, -3.0);
  assert_eq!(v1.magnitude(), 1.0);
  assert_eq!(v2.magnitude(), 1.0);
  assert_eq!(v3.magnitude(), 1.0);
  assert_eq!(v4.magnitude(), (14.0_f64).sqrt());
  assert_eq!(v5.magnitude(), (14.0_f64).sqrt());
}
#[test]
fn normalizing_vectors() {
  let v1 = vector(4.0, 0.0, 0.0);
  let v2 = vector(1.0, 2.0, 3.0);
  assert!(v1.normalize().equals(vector(1.0, 0.0, 0.0)));
  assert!(v2.normalize().equals(vector(0.26726, 0.53452, 0.80178)));
}
#[test]
fn dot_product_of_tuples() {
  let v1 = vector(1.0, 2.0, 3.0);
  let v2 = vector(2.0, 3.0, 4.0);
  assert_eq!(v1.dot(v2), 20.0);
}
#[test]
fn cross_product_of_vectors() {
  let v1 = vector(1.0, 2.0, 3.0);
  let v2 = vector(2.0, 3.0, 4.0);
  assert!(v1.cross(v2).equals(vector(-1.0, 2.0, -1.0)));
  let v2 = vector(2.0, 3.0, 4.0);
  assert!(v2.cross(v1).equals(vector(1.0, -2.0, 1.0)));
}

#[derive(Copy, Clone)]
struct Projectile {
  position: Tuple,
  velocity: Tuple
}

#[derive(Copy, Clone)]
struct Environment {
  gravity: Tuple,
  wind: Tuple
}
fn tick(env: Environment, proj: Projectile) -> Projectile {
  let position = proj.position + proj.velocity;
  let velocity = proj.velocity + env.gravity + env.wind;
  Projectile{position, velocity}
}

fn main() {
  let mut p = Projectile{position: point(0.0, 1.0, 0.0), velocity: vector(1.0, 1.0, 0.0).normalize()};
  let e = Environment{gravity: vector(0.0, -0.1, 0.0), wind: vector(-0.01, 0.0, 0.0)};
  let mut tick_count = 0;
  while p.position.y >= 0.0 {
    println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);
    p = tick(e, p);
    tick_count += 1
  }
  println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);

}

