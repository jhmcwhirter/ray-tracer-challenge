const EPSILON: f64 = 0.00001;
struct Tuple { pub x: f64, pub y: f64, pub z: f64, pub w: f64 }

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
    (self.x - t.x).abs() < EPSILON && (self.y - t.y).abs() < EPSILON && (self.z - t.z).abs() < EPSILON && (self.w - t.w).abs() < EPSILON
  }
  pub fn plus(&self, t: Tuple) -> Tuple {
    Tuple::new(self.x + t.x, self.y + t.y, self.z + t.z, self.w + t.w)
  }
}

struct Point {
  pub tuple: Tuple
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point{tuple: Tuple::new(x, y, z, 1.0)}
  }
  pub fn x(&self) -> f64 {
    self.tuple.x
  }
  pub fn y(&self) -> f64 {
    self.tuple.y
  }
  pub fn z(&self) -> f64 {
    self.tuple.z
  }
  pub fn equals(&self, p: Point) -> bool {
    self.tuple.equals(p.tuple)
  }
}

struct Vector {
  pub tuple: Tuple
}

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Vector {
    Vector{tuple: Tuple::new(x, y, z, 0.0)}
  }
  pub fn x(&self) -> f64 {
    self.tuple.x
  }
  pub fn y(&self) -> f64 {
    self.tuple.y
  }
  pub fn z(&self) -> f64 {
    self.tuple.z
  }
  pub fn equals(&self, v: Vector) -> bool {
    self.tuple.equals(v.tuple)
  }
}


#[test]
fn tuple_with_w_eq_1_is_a_point() {
  let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
  assert_eq!(tuple.x, 1.0);
  assert_eq!(tuple.y, 2.0);
  assert_eq!(tuple.z, 3.0);
  assert_eq!(tuple.w, 1.0);
  assert!(tuple.is_point());
  assert!(!tuple.is_vector());
}
#[test]
fn tuple_with_w_eq_0_is_a_vector() {
  let tuple = Tuple::new(1.0, 2.0, 3.0, 0.0);
  assert_eq!(tuple.x, 1.0);
  assert_eq!(tuple.y, 2.0);
  assert_eq!(tuple.z, 3.0);
  assert_eq!(tuple.w, 0.0);
  assert!(tuple.is_vector());
  assert!(!tuple.is_point());
}
#[test]
fn equal_tuples_are_equal() {
  let tuple1 = Tuple::new(1.0, 2.0, 3.0, 0.0);
  let tuple2 = Tuple::new(1.0, 2.0, 3.0, 0.0);
  assert!(tuple1.equals(tuple2));
}
#[test]
fn point_creates_points() {
  let point = Point::new(1.0, 2.0, 3.0);
  assert!(point.tuple.is_point())
}
#[test]
fn equal_points_are_equal() {
  let point1 = Point::new(1.0, 2.0, 3.0);
  let point2 = Point::new(1.0, 2.0, 3.0);
  assert!(point1.equals(point2));
}
#[test]
fn vector_creates_vectors() {
  let vector = Vector::new(1.0, 2.0, 3.0);
  assert!(vector.tuple.is_vector())
}
#[test]
fn equal_vectors_are_equal() {
  let vector1 = Vector::new(1.0, 2.0, 3.0);
  let vector2 = Vector::new(1.0, 2.0, 3.0);
  assert!(vector1.equals(vector2));
}
#[test]
fn adding_two_tuples() {
  let tuple1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
  let tuple2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
  assert!(tuple1.plus(tuple2).equals(Tuple::new(1.0, 1.0, 6.0, 1.0)));
}

fn main() {
  let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
  println!("tuple = ({}, {}, {}, {})", tuple.x, tuple.y, tuple.z, tuple.w);
  let point = Point::new(1.0, -2.0, -3.5);
  println!("point = ({}, {}, {})", point.x(), point.y(), point.z());
  let vector = Vector::new(-1.0, 2.6, 10.0);
  println!("vector = ({}, {}, {})", vector.x(), vector.y(), vector.z());
}

