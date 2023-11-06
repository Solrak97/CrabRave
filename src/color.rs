pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn red(&self) -> f64 {
        self.red
    }

    pub fn green(&self) -> f64 {
        self.green
    }

    pub fn blue(&self) -> f64 {
        self.blue
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
            red: self.green * rhs.blue - self.blue * rhs.green,
            green: self.blue * rhs.red - self.red * rhs.blue,
            blue: self.red * rhs.green - self.green * rhs.red,
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
}
