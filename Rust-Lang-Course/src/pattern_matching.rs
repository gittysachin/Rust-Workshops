fn how_many(x: i32) -> &'static str {
  match x {
    0 => "no",
    1 | 2 => "one or two",
    12 => "a dozen",
    9...11 => "lots of",
    _ if (x % 2 == 0) => "some",
    _ => "a few",
  }
}

pub fn pattern_matching() {
  for x in 0..13 {
    println!("{}: I have {} oranges", x, how_many(x));
  }

  let point = (3, 7);

  match (point) {
    (0, 0) => println!("origin"),
    (0, y) => println!("x-axis, y = {}", y),
    (x, 0) => println!("y-axis, x = {}", x),
    (_, y) => println!("(?, {})", y), // this one
  }

  let c: Color = Color::CmykColor {
    cyan: 0,
    magenta: 156,
    yellow: 50,
    black: 0,
  };

  match c {
    Color::Red => println!("r"),
    Color::Green => println!("g"),
    Color::Blue => println!("b"),
    Color::RgbColor(0, 0, 0) => println!("black"),
    Color::CmykColor { balck: 255, .. } => println!("black"), // this one
    Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
    _ => println!("Some other color"),
  }
}
