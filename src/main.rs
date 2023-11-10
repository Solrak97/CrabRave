mod primitive;
use primitive::{ point::Point, vector::Vector, tuple::Tuple };
mod float_utils;
mod color;
use crate::{ color::*, canvas::Canvas };
mod canvas;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Simulation
    let position = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let gravity = Vector::new(0.0, -0.1, 0.0);

    let projectile_path = fire_projectile(position, velocity, wind, gravity);

    // Sim render
    let mut canv = Canvas::new(900, 550);

    for p in projectile_path.iter() {
        let mut _y = canv.height - (p.y() as i32);
        _y = _y.clamp(0, canv.height);

        Canvas::write_pixel(&mut canv, p.x() as i32, _y, Color::red());
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

fn fire_projectile(position: Point, velocity: Vector, wind: Vector, gravity: Vector) -> Vec<Point> {
    let mut projectile_positions: Vec<Point> = Vec::new();

    let mut projectile = Projectile {
        position,
        velocity,
    };

    let env = Environment {
        wind,
        gravity,
    };

    while projectile.position.y() > 0.0 {
        projectile = tick(&env, &projectile);
        if projectile.position.y() > 0.0 {
            projectile_positions.push(projectile.position);
        }
    }

    projectile_positions
}
