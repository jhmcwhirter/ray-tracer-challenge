use std::fs;

mod tuple;
use tuple::Tuple;

mod canvas;
use canvas::Canvas;

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
  let filename = "./test.ppm";
  let color = Tuple::color(0.0, 100.0, 100.0);
  let mut canvas = Canvas::new(900, 550);
  let mut p = Projectile{position: Tuple::point(0.0, 1.0, 0.0), velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25};
  let e = Environment{gravity: Tuple::vector(0.0, -0.1, 0.0), wind: Tuple::vector(-0.01, 0.0, 0.0)};
  let mut tick_count = 0;
  while p.position.y >= 0.0 {
    println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);
    canvas.write_pixel(p.position.x as usize, 550 - p.position.y as usize, color);
    p = tick(e, p);
    tick_count += 1
  }
  println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);
  let ppm = canvas.to_ppm();
  fs::write(filename, ppm);
  
}

