use libraytracer::ray::Ray;
use libraytracer::color::Color;
use libraytracer::sphere::Sphere;
use libraytracer::canvas::Canvas;
use libraytracer::tuple::Tuple;
use libraytracer::transform::TransformBuilder;
use libraytracer::intersection::Intersect;
use libraytracer::light::PointLight;

fn main() {
    let canvas_dim = 100;
    let mut canvas = Canvas::new(canvas_dim, canvas_dim);
    let mut s = Sphere::new(1);
    s.material.color = Color::new(1.0, 0.2, 1.0);
    // s.set_transform(TransformBuilder::new(4).scale(0.5, 1.0, 1.0).shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).build());
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_color, light_position);
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half_wallsize = wall_size / 2.0;
    println!("half: {}", half_wallsize);
    let pixel_size = wall_size / canvas_dim as f32;
    println!("pixel size: {}", pixel_size);
    let red = Color::new(1.0, 0.0, 0.0);
    for i in 0..canvas_dim {
        let y = half_wallsize - i as f32 * pixel_size;
        for j in 0..canvas_dim {
            let x = -half_wallsize + pixel_size * j as f32;
            let pos = Tuple::point(x, y, wall_z);
            let r_direction = (pos - ray_origin).normalize();
            let r = Ray::new(ray_origin.clone(), r_direction);
            if let Some(h) = s.intersect(&r).hit() {
                let point = r.position(h.point());
                let normal = h.object().normal_at(point);
                let eye = -(*r.direction());
                let color = s.material.lighting(&light, &point, &eye, &normal);
                canvas.write_pixel(j as usize, i as usize, color);
            }
        }
    }
    canvas.save("sphere_cast.ppm".to_string()).unwrap();
}