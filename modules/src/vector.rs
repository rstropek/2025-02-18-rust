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