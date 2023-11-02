mod tuple;
use tuple::*;

fn main() {
    let mut projectile: Projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector_normalize(&vector(1.0, 1.0, 0.0)),
    };

    let env: Environment = Environment { wind: vector(0.0, -0.1, 0.0), gravity: vector(-0.01, 0.0, 0.0) };

    for n in 0..1000 {
        projectile = tick(&env, &projectile);
        println!("{:?}", projectile.position);
    }
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