union IntOrFloat {
  i: i32,
  f: f32,
}

fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat { i: 42 } => println!("meaning of life "),
      IntOrFloat { f } => println!("f32 = {}", f),
    }
  }
}

pub fn unions() {
  let mut iof = IntOrFloat { i: 123 };

  unsafe {
    iof.i = 42; // if we'll change this value from 42 to something else than the answer will a very small positive number in decimals. Because it will take it as f
  }

  let value = unsafe { iof.i };

  process_value(iof);
  process_value(IntOrFloat { f: 1.23 });
}
