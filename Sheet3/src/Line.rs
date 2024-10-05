use crate::Point::Point;

pub struct Line {
    start: Point,
    end: Point,
    m: f32,
    q: f32,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let m = (end.y - start.y) / (end.x - start.x);
        let q = start.y - m * start.x;
        Self { start, end, m, q }
    }

    pub fn contains(self, point: &Point) -> Result< (), String>{
        let y_t= self.m * point.x + self.q;
        if y_t == point.y {
            Ok(())
        }else{
            Err(String::from("Il punto non appartiene alla line"))
        }
    }
}