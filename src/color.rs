use std::ops::{Add, Sub, Mul};
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::cmp;
use super::utils;

#[derive(Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {red, green, blue}
    }

    pub fn black() -> Self {
        Self {red: 0.0, green: 0.0, blue: 0.0}
    }

    pub fn red(&self) -> f32 {
        self.red
    }

    pub fn green(&self) -> f32 {
        self.green
    }

    pub fn blue(&self) -> f32 {
        self.blue
    }

    fn scale_color(&self) -> (u8, u8, u8) {
        let scaled_red = cmp::min(255, (self.red * 255f32).ceil() as u8);
        let scaled_green = cmp::min(255, (self.green * 255f32).ceil() as u8);
        let scaled_blue = cmp::min(255, (self.blue * 255f32).ceil() as u8);
        return (scaled_red, scaled_green, scaled_blue);
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, _rhs: Color) -> Self {
        Self {red: self.red + _rhs.red, green: self.green + _rhs.green, blue: self.blue + _rhs.blue}
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, _rhs: Color) -> Self {
        Self {red: self.red - _rhs.red, green: self.green - _rhs.green, blue: self.blue - _rhs.blue}
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, _rhs: Color) -> Self {
        Self {red: self.red * _rhs.red, green: self.green * _rhs.green, blue: self.blue * _rhs.blue}
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;
    fn mul(self, _rhs: &Color) -> Color {
        Color {red: self.red * _rhs.red, green: self.green * _rhs.green, blue: self.blue * _rhs.blue}
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, _rhs: f32) -> Self {
        Self {red: self.red * _rhs, green: self.green * _rhs, blue: self.blue * _rhs}
    }
}

impl Mul<f32> for &Color {
    type Output = Color;
    fn mul(self, _rhs: f32) -> Color {
        Color {red: self.red * _rhs, green: self.green * _rhs, blue: self.blue * _rhs}
    }
}

impl Display for Color {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let (r, g, b) = self.scale_color();
        write!(fmt, "{} {} {}", r, g, b)
    }
}

impl Debug for Color {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let (r, g, b) = self.scale_color();
        write!(fmt, "{} {} {}", r, g, b)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return utils::is_equal(self.red, other.red) &&
            utils::is_equal(self.green, other.green) &&
            utils::is_equal(self.blue, other.blue);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let c = Color::new(1.0, 2.0, 3.0);
        assert_eq!(c.red, 1.0);
        assert_eq!(c.green, 2.0);
        assert_eq!(c.blue(), 3.0);
    }

    #[test]
    fn test_color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.1, 0.4, 0.25);
        let result = c1 + c2;
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_color_sub() {
        let c1 = Color::new(0.8, 0.6, 0.75);
        let c2 = Color::new(0.3, 0.6, 0.25);
        let result = c1 - c2;
        assert_eq!(result, Color::new(0.5, 0.0, 0.5));
    }

    #[test]
    fn test_color_mul() {
        let c1 = Color::new(0.2, 0.4, 0.8);
        let c2 = Color::new(0.5, 0.5, 0.5);
        let result = c1 * c2;
        assert_eq!(result, Color::new(0.1, 0.2, 0.4));
    }

    #[test]
    fn test_color_scaler_mul() {
        let c1 = Color::new(0.2, 0.4, 0.8);
        let result = c1 * 5 as f32;
        assert_eq!(result, Color::new(1.0, 2.0, 4.0))
    }
}