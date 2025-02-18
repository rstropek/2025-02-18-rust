pub struct Vector2d {
    x: f64,
    y: f64,
}

impl Vector2d {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2d { x, y }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn add(&self, other: &Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(&self, factor: f64) -> Vector2d {
        Vector2d {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_new() {
        let v = Vector2d::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_getters() {
        let v = Vector2d::new(3.0, 4.0);
        assert_eq!(v.get_x(), 3.0);
        assert_eq!(v.get_y(), 4.0);
    }

    #[test]
    fn test_length() {
        let v = Vector2d::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_add() {
        let v1 = Vector2d::new(1.0, 2.0);
        let v2 = Vector2d::new(3.0, 4.0);
        let result = v1.add(&v2);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_scale() {
        let v = Vector2d::new(2.0, 3.0);
        let result = v.scale(2.0);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }
} 