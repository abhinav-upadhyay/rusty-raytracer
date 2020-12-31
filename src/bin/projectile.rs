use libraytracer::tuple::Tuple;
use libraytracer::canvas::Canvas;
use libraytracer::color::Color;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple
}

struct Env {
    gravity: Tuple,
    wind: Tuple
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Self {position, velocity}
    }
}

impl Env {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self {gravity, wind}
    }

    pub fn tick(&self, projectile: Projectile) -> Projectile {
        Projectile {position: projectile.position + &projectile.velocity, velocity: projectile.velocity + &self.gravity + &self.wind}
    }
}

fn main() {
    println!("Running projectile!");
    let mut proj = Projectile::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25);
    let env = Env::new(Tuple::vector(0.0, -0.1, 0.0), Tuple::vector(-0.01, 0.0, 0.0));
    let mut canvas = Canvas::new(900, 550);
    loop {
        proj = env.tick(proj);
        let x = proj.position.x().round() as usize;
        let y = canvas.width() as usize - proj.position.y().round() as usize;
        canvas.write_pixel(x, y, Color::new(1.0, 0.0, 0.0));
        println!("new proj: {:?}", proj);
        if proj.position.y() <= 0.0 {
            break;
        }
    }
    canvas.save("canvas.ppm".to_string()).unwrap();
}
