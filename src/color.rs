use std::ops::{Add, Mul, Neg, Sub, Div};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(r, g, b)
    }

    pub fn r(self) -> f64 {self.0}
    pub fn g(self) -> f64 {self.1}
    pub fn b(self) -> f64 {self.2}
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color::new(self * color.r(), self * color.g(), self * color.b())
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl Div<Color> for f64 {
    type Output = Color;

    fn div(self, color: Color) -> Color {
        Color::new(self / color.r(), self / color.g(), self / color.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(1.0, 0.5, 0.25);
        assert_eq!(color.r(), 1.0);
        assert_eq!(color.g(), 0.5);
        assert_eq!(color.b(), 0.25);
    }

    #[test]
    fn test_add() {
        let color1 = Color::new(1.0, 0.5, 0.25);
        let color2 = Color::new(0.5, 0.25, 0.125);
        let result = color1 + color2;
        assert_eq!(result.r(), 1.5);
        assert_eq!(result.g(), 0.75);
        assert_eq!(result.b(), 0.375);
    }

    #[test]
    fn test_sub() {
        let color1 = Color::new(1.0, 0.5, 0.25);
        let color2 = Color::new(0.5, 0.25, 0.125);
        let result = color1 - color2;
        assert_eq!(result.r(), 0.5);
        assert_eq!(result.g(), 0.25);
        assert_eq!(result.b(), 0.125);
    }

    #[test]
    fn test_neg() {
        let color = Color::new(1.0, 0.5, 0.25);
        let result = -color;
        assert_eq!(result.r(), -1.0);
        assert_eq!(result.g(), -0.5);
        assert_eq!(result.b(), -0.25);
    }

    #[test]
    fn test_mul_colors() {
        let color1 = Color::new(1.0, 0.5, 0.25);
        let color2 = Color::new(0.5, 0.25, 4.0);
        let result = color1 * color2;
        assert_eq!(result.r(), 0.5);
        assert_eq!(result.g(), 0.125);
        assert_eq!(result.b(), 1.0);
    }

    #[test]
    fn test_mul_color() {
        let color = Color::new(1.0, 0.5, 0.25);
        let result = color * 2.0;
        assert_eq!(result.r(), 2.0);
        assert_eq!(result.g(), 1.0);
        assert_eq!(result.b(), 0.5);
    }

    #[test]
    fn test_div_color() {
    let color = Color::new(1.0, 0.5, 0.25);
        let result = color / 2.0;
        assert_eq!(result.r(), 0.5);
        assert_eq!(result.g(), 0.25);
        assert_eq!(result.b(), 0.125);
    }

}
