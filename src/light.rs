use super::tuple::Tuple;
use super::color::Color;

#[derive(Debug, PartialEq)]
pub struct PointLight {
    intensity: Color,
    position: Tuple
}

impl PointLight {
    pub fn new(intensity: Color, position: Tuple) -> Self {
        PointLight {intensity, position}
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
    }

    pub fn position(&self) -> &Tuple {
        &self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_light() {
        let color = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(1.0, 0.0, 0.0);
        let l = PointLight::new(color.clone(), position);
        assert_eq!(*l.intensity(), color);
        assert_eq!(*l.position(), position);
    }
}