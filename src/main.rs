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
  let mut p = Projectile{position: Tuple::point(0.0, 1.0, 0.0), velocity: Tuple::vector(1.0, 1.0, 0.0).normalize()};
  let e = Environment{gravity: Tuple::vector(0.0, -0.1, 0.0), wind: Tuple::vector(-0.01, 0.0, 0.0)};
  let mut tick_count = 0;
  while p.position.y >= 0.0 {
    println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);
    p = tick(e, p);
    tick_count += 1
  }
  println!("tick: {}, position(x,y): ({}, {})", tick_count, p.position.x, p.position.y);

}

