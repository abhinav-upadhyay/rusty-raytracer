use libraytracer::tuple::Tuple;
use libraytracer::canvas::Canvas;
use libraytracer::color::Color;
use libraytracer::transform::TransformBuilder;
use std::f32::consts::PI;

fn main() {
    println!("Running clock!");
    let mut canvas = Canvas::new(400, 400);
    let twelve = Tuple::point(0.0, 0.0, 1.0);
    let radius = 400 * 3 / 8;
    for i in 0..12 {
        let transform = TransformBuilder::new(4).rotate_y(i as f32 * PI / 6.0).build();
        let p = (&transform * &twelve).unwrap() * radius as f32;
        let final_p = Tuple::point(p.x(), p.z(), 0.0) + Tuple::point(200.0, 200.0, 0.0);
        let x = final_p.x().round() as usize;
        let y = final_p.y().round() as usize;
        println!("writing pixel: {} {}, {}",i, x, y);
        canvas.write_pixel(x, y, Color::new(1.0, 0.0, 0.0));
    }
    canvas.save("clock.ppm".to_string()).unwrap();
}