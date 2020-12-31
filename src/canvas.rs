use super::color::Color;
use std::fs;

pub struct Canvas {
    width: u16,
    height: u16,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let pixels = vec![Color::new(0.0, 0.0, 0.0); height as usize * width as usize];
        Self {width, height, pixels}
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel_idx : usize = (y * self.width as usize + x).into();
        if pixel_idx >= self.pixels.len() {
            println!("Attempt to update pixel index {} for width: {}, height: {}, x: {}, y: {}. Ignoring",
            pixel_idx, self.width, self.height, x, y);
            return;
        }
        self.pixels[pixel_idx] = color;
    }

    pub fn gen_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    pub fn gen_ppm_body(&self) -> String {
        let mut lines : Vec<String> = Vec::new();
        for i in (0..self.pixels.len()).step_by(self.width as usize) {
            let pixels_row = self.pixels.get(i..i + self.width as usize).unwrap();
            let mut output_row : String = "".to_string();
            for c in pixels_row {
                let color_str : String = format!("{}", c);
                if output_row.len() + color_str.len() + 1 > 70 {
                    lines.push(output_row);
                    output_row = color_str;
                } else {
                    if output_row.len() != 0 {
                        output_row.push(' ');
                    }
                    output_row.push_str(color_str.as_str());
                }
            }
            lines.push(output_row);
        }
        lines.push("".to_string());
        return lines.join("\n");
    }

    pub fn save(&self, path: String) -> std::io::Result<()> {
        let mut contents = self.gen_ppm_header();
        contents.push_str(self.gen_ppm_body().as_str());
        fs::write(path, contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_canvas_new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);
        for c in canvas.pixels {
            assert_eq!(c, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn test_write_piexl() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        assert_eq!(canvas.pixels[32], Color::new(1.0, 0.0, 0.0));
        for (i, c) in canvas.pixels.iter().enumerate() {
            if i == 32 {
                continue;
            }
            assert_eq!(c, &Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn test_gen_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let expected_header = "P3\n5 3\n255\n";
        assert_eq!(expected_header, canvas.gen_ppm_header());
    }

    #[test]
    fn test_ppm_body() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        println!("c2: {}, c3: {}", c2, c3);
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        let ppm_body = canvas.gen_ppm_body();
        let expected_body = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(expected_body, ppm_body);
    }

    #[test]
    fn test_split_long_ppm_lines() {
        let mut canvas = Canvas::new(10, 2);
        for i in 0..20 {
            canvas.pixels[i] = Color::new(1.0, 0.8, 0.6);
        }
        let ppm_body = canvas.gen_ppm_body();
        let expected_body = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        assert_eq!(expected_body, ppm_body);
    }

    #[test]
    fn test_terminating_new_line() {
        let c = Canvas::new(5, 3);
        let body = c.gen_ppm_body();
        assert_eq!(str::ends_with(body.as_str(), "\n"), true);
    }

}