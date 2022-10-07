use std::fs;
use std::f64::consts::PI;

mod tuple;
use tuple::Tuple;

mod canvas;
use canvas::Canvas;

mod matrix;
use matrix::Matrix;

mod ray;
mod sphere;
mod transformations;

// Clock

fn main() {
  fn rad(deg: f64) -> f64 {
    (deg / 180.0) * PI
  }
  let filename = "./test.ppm";
  let color_x = Tuple::color(0.0, 100.0, 100.0);
  let mut canvas = Canvas::new(500, 500);
  let start = Tuple::point(0.0, -150.0, 0.0);
  
  let mut points = [Tuple::point(0.0, 0.0, 0.0); 14];
  points[0] = start;

  let rotation_matrix = Matrix::identity().rotate_z(PI/6.0);
  let mut next = start;
  for point in 1..13 {
    let rot = rotation_matrix.clone();
    next = rot * points[point - 1];
    points[point] = next;
  }

  let translation_matrix = Matrix::identity().translate(250.0, 250.0, 0.0);
  for mut clock_point in points {
    let trans = translation_matrix.clone();
    clock_point = trans * clock_point;
    canvas.write_pixel(clock_point.x, clock_point.y, color_x);
    canvas.write_pixel(clock_point.x + 1.0, clock_point.y, color_x);
    canvas.write_pixel(clock_point.x, clock_point.y + 1.0, color_x);
    canvas.write_pixel(clock_point.x + 1.0, clock_point.y + 1.0, color_x);
  }

  let ppm = canvas.to_ppm();
  fs::write(filename, ppm);
}

// Projectile

// #[derive(Copy, Clone)]
// struct Projectile {
//   position: Tuple,
//   velocity: Tuple
// }

// #[derive(Copy, Clone)]
// struct Environment {
//   gravity: Tuple,
//   wind: Tuple
// }
// fn tick(env: Environment, proj: Projectile) -> Projectile {
//   let position = proj.position + proj.velocity;
//   let velocity = proj.velocity + env.gravity + env.wind;
//   Projectile{position, velocity}
// }

// fn main() {
//   let filename = "./test.ppm";
//   let color = Tuple::color(0.0, 100.0, 100.0);
//   let mut canvas = Canvas::new(900, 550);
//   let mut p = Projectile{position: Tuple::point(0.0, 1.0, 0.0), velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.0};
//   let e = Environment{gravity: Tuple::vector(0.0, -0.1, 0.0), wind: Tuple::vector(-0.01, 0.0, 0.0)};
//   let mut tick_count = 0;
//   while p.position.y >= 0.0 {
//     println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);
//     canvas.write_pixel(p.position.x as usize, 550 - p.position.y as usize, color);
//     canvas.write_pixel((p.position.x + 1.0) as usize, 550 - p.position.y as usize, color);
//     canvas.write_pixel(p.position.x as usize, 550 - ((p.position.y + 1.0) as usize), color);
//     canvas.write_pixel((p.position.x + 1.0) as usize, 550 - ((p.position.y + 1.0) as usize), color);

//     p = tick(e, p);
//     tick_count += 1
//   }
//   println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);
//   let ppm = canvas.to_ppm();
//   fs::write(filename, ppm);
  
// }

