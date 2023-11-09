mod primitive;
use primitive::{ point::Point, vector::Vector, tuple::Tuple };
mod float_utils;
mod color;
use crate::{color::*, canvas::Canvas};
mod canvas;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
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
    println!("{:?}", count);





    let mut canv = Canvas::new(10, 10);
    
    for x in 0..10 {
        for y in 0..10 {
            if x == y {
                Canvas::write_pixel(&mut canv, x, y, Color::red())
            }
        }
    }


    let mut f = File::create("test.ppm")?;
    f.write_all(Canvas::canvas_to_ppm(&canv).as_bytes())?;

    Ok(())


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
