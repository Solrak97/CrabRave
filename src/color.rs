use crate::float_utils::floats_equal;

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn _red(&self) -> f64 {
        self.red
    }

    pub fn _green(&self) -> f64 {
        self.green
    }

    pub fn _blue(&self) -> f64 {
        self.blue
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

// Custom partial eq
impl PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        floats_equal(self.red, rhs.red) &&
            floats_equal(self.green, rhs.green) &&
            floats_equal(self.blue, rhs.blue)
    }
}

// col + col
impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Color {
        Color { red: self.red + rhs.red, green: self.green + rhs.green, blue: self.blue + rhs.blue }
    }
}

// col - col
impl std::ops::Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Color {
        Color { red: self.red - rhs.red, green: self.green - rhs.green, blue: self.blue - rhs.blue }
    }
}

// col * scalar
impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color { red: self.red * rhs, green: self.green * rhs, blue: self.blue * rhs }
    }
}

// scalar * col
impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}

// col * col
impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn is_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(c.red == -0.5 && c.green == 0.4 && c.blue == 1.7);
    }

    #[test]
    fn col_sum() {
        let c1 = Color::new(0.1, 0.3, 0.2);
        let c2 = Color::new(0.2, 0.7, 0.3);

        assert_eq!(Color::new(0.3, 1.0, 0.5), c1 + c2)
    }

    #[test]
    fn col_sub() {
        let c1 = Color::new(0.1, 0.3, 0.2);
        let c2 = Color::new(0.2, 0.7, 0.3);

        assert_eq!(Color::new(-0.1, -0.4, -0.1), c1 - c2)
    }

    #[test]
    fn col_mul_scalar() {
        let c = Color::new(0.1, 0.3, 0.2);
        let s = 5.0;
        assert_eq!(Color::new(0.5, 1.5, 1.0), c * s)
    }

    #[test]
    fn scalar_mul_col() {
        let c = Color::new(0.1, 0.3, 0.2);
        let s = 5.0;
        assert_eq!(Color::new(0.5, 1.5, 1.0), s * c)
    }

    #[test]
    fn col_mul_col() {
        let c1 = Color::new(0.1, 0.3, 0.2);
        let c2 = Color::new(0.2, 0.7, 0.3);

        assert_eq!(Color::new(0.02, 0.21, 0.06), c1 * c2)
    }
}
