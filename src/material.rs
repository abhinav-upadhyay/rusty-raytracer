use super::color::Color;
use super::light::PointLight;
use super::tuple::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

impl Material {
    pub fn new() -> Self {
        Self {color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0}
    }

    pub fn lighting(&self, light: &PointLight, position: &Tuple, eyev: &Tuple, normalv: &Tuple) -> Color {
        let effective_color = &self.color * light.intensity();
        let lightv = (light.position() - position).normalize();
        let ambient = &effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);
        let mut diffuse = Color::black();
        let mut specular = Color::black();
        if light_dot_normal >= 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = eyev.dot(&reflectv);
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor;
            }
        }
        return ambient + diffuse + specular;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_between_light_and_surface_and_eye_at_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let val = 2f32.sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, val, val);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_eye_opposite_surface_and_light_at_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 10.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let val = 2f32.sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, -val, -val);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 10.0, -10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, 10.0));
        let result = m.lighting(&light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}

