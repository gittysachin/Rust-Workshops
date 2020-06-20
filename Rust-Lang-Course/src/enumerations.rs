use std::mem;

enum Color {
  Red,
  Green,
  Blue,
  RgbColor(u8, u8, u8), // tuple
  CmykColor {
    cyan: u8,
    magenta: u8,
    yellow: u8,
    black: u8,
  }, // struct
}

fn enums() {
  // let c: COlor = Color::Red;
  // let c: Color = Color::RgbColor(10, 255, 56);
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
    Color::CmykColor {
      cyan: _,
      magenta: _,
      yellow: _,
      balck: 255,
    } => println!("black"),
    Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
    _ => println!("Some other color"),
  }
}
