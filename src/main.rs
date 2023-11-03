mod tuple;
pub mod primitive;
mod Primitive;
use tuple::*;

mod colors;
use colors::*;



fn main() {
    let mut projectile: Projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: scale_tuple(&vector_normalize(&vector(1.0, 1.0, 0.0)), 5.0),
    };

    let env: Environment = Environment { wind: vector(0.0, -0.1, 0.0), gravity: vector(-0.01, 0.0, 0.0) };

    let mut count = 0;
    while !floored(&projectile.position){
        count += 1;
        projectile = tick(&env, &projectile);
    }
    println!("{:?}", count)
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    wind: Tuple,
    gravity: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position: Tuple = add_tuple(&proj.position, &proj.velocity);
    let velocity: Tuple = add_tuple(&add_tuple(&proj.velocity, &env.gravity), &env.wind);
    Projectile { position: position, velocity: velocity}
}