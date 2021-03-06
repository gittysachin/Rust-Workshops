struct Point {
  x: f64,
  y: f64,
}

struct Line {
  start: Point,
  end: Point,
}

impl Line {
  fn len(&self) -> f64 {
    let dx = self.end.x - self.start.x;
    let dy = self.end.y - self.start.y;
    (dx * dx + dy * dy).sqrt()
  }
}

fn methods() {
  let p1 = Point { x: 3.0, y: 4.0 };
  let p2 = Point { x: 5.0, y: 11.0 };
  let myline = Line { start: p1, end: p2 };

  println!("Lenght = {}", myline.len());
}
