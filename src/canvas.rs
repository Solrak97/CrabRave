use crate::color::*;

#[derive(Clone, Debug)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    canvas: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let canvas = vec![Color::black(); (width * height) as usize];
        Canvas { width, height, canvas }
    }

    pub fn write_pixel(cnvs: &mut Canvas, x: i32, y: i32, color: Color) {
        let coord = transform_coords(cnvs, x, y);
        cnvs.canvas[coord] = color;
    }

    pub fn pixel_at(cnvs: &Canvas, x: i32, y: i32) -> Color {
        let coord = transform_coords(cnvs, x, y);
        cnvs.canvas[coord]
    }

    pub fn canvas_to_ppm(canvas: &Canvas) -> String {
        let color_scale = 255;
        let max_chars_on_row = 69;
        let header = format!("P3\n{} {}\n{}\n", canvas.width, canvas.height, color_scale);
        let mut pix_data = "".to_owned();
        let mut pix_row = "".to_owned();

        for pixel in &canvas.canvas {
            let scaled_pixel = *pixel * (color_scale as f64);
            let scaled_red = scaled_pixel._red().clamp(0.0, color_scale as f64) as i32;
            let scaled_green = scaled_pixel._green().clamp(0.0, color_scale as f64)  as i32;
            let scaled_blue = scaled_pixel._blue().clamp(0.0, color_scale as f64)  as i32;

            let pix_value = &format!(
                "{} {} {} ",
                scaled_red,
                scaled_green,
                scaled_blue
            ).to_string();

            if pix_row.len() + pix_value.len() > max_chars_on_row {
                pix_row += "\n";
                pix_data += &pix_row.to_string();
                pix_row = "".to_owned();
            }

            pix_row += pix_value;
        }
        pix_data += &pix_row.to_string();

        return header + &pix_data.to_string();
    }
}

fn transform_coords(c: &Canvas, x: i32, y: i32) -> usize {
    (y * c.width + x) as usize
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    use super::{ Canvas, transform_coords };

    #[test]
    fn new_canvas() {
        let cnvs = Canvas::new(1280, 720);

        let is_width = cnvs.width == 1280;
        let is_height = cnvs.height == 720;

        let mut is_black = true;

        let black = Color::black();

        print!("{}, {}", is_width, is_height);
        for x in 0..cnvs.width {
            for y in 0..cnvs.height {
                let p = Canvas::pixel_at(&cnvs, x, y);
                is_black = p == black;
            }
        }

        assert_eq!((true, true, true), (is_width, is_height, is_black))
    }

    #[test]
    fn _transform_coords() {
        let x = 5;
        let y = 7;
        let c = Canvas::new(10, 12);

        let tc = transform_coords(&c, x, y);

        assert_eq!(75, tc)
    }

    #[test]
    fn _write_pixel() {
        let mut c = Canvas::new(10, 10);
        let pink = Color::new(0.9, 0.5, 0.6);
        Canvas::write_pixel(&mut c, 4, 7, pink);

        let coord = transform_coords(&c, 4, 7);
        let p = c.canvas[coord];

        assert_eq!(p, pink)
    }

    #[test]
    fn pixel_at() {
        let mut c = Canvas::new(10, 10);
        let pink = Color::new(0.9, 0.5, 0.6);
        Canvas::write_pixel(&mut c, 4, 7, pink);

        let p = Canvas::pixel_at(&c, 4, 7);

        assert_eq!(p, pink)
    }

    #[test]
    fn _canvas_to_ppm() {
        let ppm_data =
            "P3
10 10
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
255 0 0 ";

        let mut canv = Canvas::new(10, 10);

        for x in 0..10 {
            for y in 0..10 {
                if x == y {
                    Canvas::write_pixel(&mut canv, x, y, Color::red());
                }
            }
        }

        let ppm_gen_data = Canvas::canvas_to_ppm(&canv);

        assert_eq!(ppm_gen_data, ppm_data)
    }
}
