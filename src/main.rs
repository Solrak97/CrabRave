mod primitive;
use primitive::{ point::Point, vector::Vector, tuple::Tuple };

mod color;
use color::*;

fn main() {
    let mut projectile: Projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize()
    };

    let env: Environment = Environment {
        wind: Vector::new(0.0, -0.1, 0.0),
        gravity: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut count = 0;
    while projectile.position.y() > 0.0 {
        count += 1;
        projectile = tick(&env, &projectile);
        println!("{:?}", projectile.position);
    }
    println!("{:?}", count)
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    wind: Vector,
    gravity: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position: Point = proj.position + proj.velocity;
    let velocity: Vector = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}
