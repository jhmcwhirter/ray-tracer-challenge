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
  pub fn red(&self) -> f64 {
    self.x
  }
  pub fn green(&self) -> f64 {
    self.y
  }
  pub fn blue(&self) -> f64 {
    self.z
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

// constructors
fn point(x: f64, y: f64, z: f64) -> Tuple {
  Tuple::new(x, y, z, 1.0)
}
fn vector(x: f64, y: f64, z: f64) -> Tuple {
  Tuple::new(x, y, z, 0.0)
}
fn color(red: f64, green: f64, blue: f64) -> Tuple {
  Tuple::new(red, green, blue, 0.0) // colors are also vectors, this might be a problem later
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
#[test]
fn colors_are_tuples() {
  let c = color(-0.5, 0.4, 1.7);
  assert_eq!(c.red(), -0.5);
  assert_eq!(c.green(), 0.4);
  assert_eq!(c.blue(), 1.7);
}
#[test]
fn adding_colors() {
  let c1 = color(0.9, 0.6, 0.75);
  let c2 = color(0.7, 0.1, 0.25);
  assert!((c1 + c2).equals(color(1.6, 0.7, 1.0)));
}
#[test]
fn subtracting_colors() {
  let c1 = color(0.9, 0.6, 0.75);
  let c2 = color(0.7, 0.1, 0.25);
  assert!((c1 - c2).equals(color(0.2, 0.5, 0.5)));
}
#[test]
fn multipyling_a_color_by_a_scalar() {
  let c = color(0.2, 0.3, 0.4);
  assert!((c * 2.0).equals(color(0.4, 0.6, 0.8)));
}

// Canvas

#[derive(Clone)]
struct Canvas{ width: usize, length: usize, matrix: Vec<Vec<Tuple>>}
impl Canvas{
  pub fn iter(&self) -> CanvasIter<'_> {
    CanvasIter{canvas: self, col: 0, row: 0}
  }
  pub fn new( width: usize, length: usize) -> Self {
    let row = vec![color(0.0, 0.0, 0.0); width];
    let matrix = vec![row; length];
    Canvas{ width: width, length: length, matrix: matrix }
  }
  pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
    self.matrix[y][x]
  }
  pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
    self.matrix[y][x] = color;
  }

  pub fn fill_with(&mut self, color: Tuple) {
    for col in 0..self.width {
      for row in 0..self.length {
        self.write_pixel(col, row, color);
      }
    }
  }

  // generate a ppm file
  pub fn to_ppm(&self) -> String {
    fn constrain(color: f64) -> i32 {
      const MAX: f64 = 255.00;
      const MIN: f64 = 0.0;
      let mut out = (color * MAX).ceil();
      if out > MAX {
        out = MAX;
      }
      if out < MIN {
        out = MIN
      }
      out as i32
    }
    let mut ppm = "P3\n".to_owned() + &self.width.to_string() + &" " + &self.length.to_string() + &"\n255\n";
    let mut col = 0;
    for p in self.iter() {
      
      if col == self.width {
        ppm += &"\n";
        col = 0
      }
      println!("{}", col);
      let red = constrain(p.color.red()).to_string() + " ";
      let green = constrain(p.color.green()).to_string() + " ";
      let blue = constrain(p.color.blue()).to_string() + " ";
      ppm += &red;
      ppm += &green;
      ppm += &blue;
      col += 1
    }
    ppm
  }
}
struct Pixel { x: usize, y: usize, color: Tuple }
struct CanvasIter<'a> { canvas: &'a Canvas, col: usize, row: usize }
impl Iterator for CanvasIter<'_> {
  type Item = Pixel;

  fn next(&mut self) -> Option<Self::Item> {
    if self.col == self.canvas.width {
      self.row += 1;
      self.col = 0;
    }
    if self.row == self.canvas.length {
      return None
    }
    let value = Some(Pixel{x: self.col, y: self.row, color: self.canvas.matrix[self.row][self.col]});
    self.col += 1;
    value
  }
}

#[test]
fn creating_a_canvas() {
  let c = Canvas::new(10, 20);
  assert_eq!(c.width, 10);
  assert_eq!(c.length, 20);
  // all the pixels are black
  for pixel in c.iter() { 
    assert!(pixel.color.equals(color(0.0, 0.0, 0.0)));
  }
}
#[test]
fn writing_pixels_to_a_canvas() {
  let mut c = Canvas::new(10, 20);
  let red = color(1.0, 0.0, 0.0);
  c.write_pixel(2, 3, red);
  assert!(c.pixel_at(2,3).equals(red));
}
#[test]
fn constructing_the_ppm_header() {
  let c = Canvas::new(5, 3);
  let ppm = c.to_ppm();
  let lines: Vec<&str> = ppm.split("\n").collect();
  assert_eq!(lines[0], "P3");
  assert_eq!(lines[1], "5 3");
  assert_eq!(lines[2], "255");
}
#[test]
fn constructing_the_ppm_pixel_data() {
  let mut c = Canvas::new(5, 3);
  let c1 = color(1.5, 0.0, 0.0);
  let c2 = color(0.0, 0.5, 0.0);
  let c3 = color(-0.5, 0.0, 1.0);
  c.write_pixel(0, 0, c1);
  c.write_pixel(2, 1, c2);
  c.write_pixel(4, 2, c3);
  let ppm = c.to_ppm();
  let lines: Vec<&str> = ppm.split("\n").collect();
  assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ");
  assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 ");
  assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 ");
}
#[test]
fn splitting_long_lines_in_ppm_files() {
  let mut c = Canvas::new(10, 2);
  let c1 = color(1.0, 0.8, 0.6);
  c.fill_with(c1);
  let ppm = c.to_ppm();
  let lines: Vec<&str> = ppm.split("\n").collect();
  assert_eq!(lines[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
  assert_eq!(lines[4], "153 255 204 153 255 204 153 255 204 153 255 204 153");
  assert_eq!(lines[5], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
  assert_eq!(lines[6], "153 255 204 153 255 204 153 255 204 153 255 204 153");
}

// Projectile

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

