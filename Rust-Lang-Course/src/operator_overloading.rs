#[derive(Debug)]

struct Point {
  x: f64,
  y: f64,
}

use std::ops::Add;

impl Add for Point {
  fn add(self, other: Point) -> Point {
    type Output = Point;
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

fn op_ov() {
  let p = Point { x: 1.0, y: 2.2 };
  let p2 = Point { x: 3.0, y: 4.0 };

  let p3 = p + p2;
}
