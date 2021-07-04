use ray;

#[derive(Debug)]
struct Projectile {
    position: ray::ray_tuple::RayTuple,
    velocity: ray::ray_tuple::RayTuple
}

struct Environment {
    gravity: ray::ray_tuple::RayTuple,
    wind: ray::ray_tuple::RayTuple
}


fn main() {
    let p = (0.0, 1.0, 0.0);
    let v = (1.0, 1.0, 0.0);
    let g = (0.0, -0.1, 0.0);
    let w = (-0.01, 0.0, 0.0);

    let mut proj = Projectile {
        position: ray::ray_tuple::RayTuple::point(p.0, p.1, p.2),
        velocity: ray::ray_tuple::RayTuple::vector(v.0, v.1, v.2)
    };

    let env = Environment {
        gravity: ray::ray_tuple::RayTuple::vector(g.0, g.1, g.2),
        wind: ray::ray_tuple::RayTuple::vector(w.0, w.1, w.2)
    };


    print_proj(&proj);
    loop {
        proj = tick(&env, &proj);
        if proj.position.y <= 0.0 {
            break;
        }
        print_proj(&proj);
    }
}

fn print_proj(proj: &Projectile) -> () {
    println!("x={} y={} z={}", proj.position.x, proj.position.y, proj.position.z);
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let pos = proj.position.clone() + proj.velocity.clone();
    let vel = proj.velocity.clone() + env.gravity.clone() + env.wind.clone();
    Projectile{
        position: pos,
        velocity: vel
    }
}
