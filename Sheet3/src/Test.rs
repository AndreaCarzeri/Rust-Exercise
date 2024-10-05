use crate::Line::Line;
use crate::Point::Point;

pub fn test() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let line = Line::new(p1, p2);

    let test_point = Point { x: 2.0, y: 3.0 };

    match line.contains(&test_point) {
        Ok(()) => println!("Il punto ({}, {}) è sulla linea.", test_point.x, test_point.y),
        Err(v) => println!("Il punto ({}, {}) NON è sulla linea.", test_point.x, test_point.y)
    }
}