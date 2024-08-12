#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn add(&self, v: &Vector2) -> Vector2 {
        Vector2::new(&self.x + v.x, &self.y + v.y)
    }

    pub fn sub(&self, v: &Vector2) -> Vector2 {
        Vector2::new(&self.x - v.x, &self.y - v.y)
    }

    pub fn mul(&self, v: &Vector2) -> Vector2 {
        Vector2::new(&self.x * v.x, &self.y * v.y)
    }

    pub fn div(&self, v: &Vector2) -> Vector2 {
        Vector2::new(&self.x / v.x, &self.y / v.y)
    }

    pub fn scale(&self, x: f64) -> Vector2 {
        Vector2::new(&self.x * x, &self.y * x)
    }

    pub fn length(&self) -> f64 {
       (&self.x * &self.x + &self.y * &self.y).sqrt()
    }
    pub fn norm(&self) -> Vector2 {
        let l = self.length();
        Vector2::new(&self.x / l, &self.y / l)
    }

    pub fn distanceTo(&self, v: &Vector2) -> f64 {
        self.sub(v).length()
    }
}
