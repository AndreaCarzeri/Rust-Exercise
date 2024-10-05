pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn new() -> Self {
        Self{x: 0.0, y: 0.0}
    }

    pub fn distance(self, point: &Point) -> f32 {
        ((self.x - point.x).powf(2.0) + (self.y - point.y).powf(2.0)).sqrt()
    }
}