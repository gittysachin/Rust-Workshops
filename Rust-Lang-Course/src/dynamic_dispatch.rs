use std::mem;

trait Printable {
  fn format(&self) -> String;
}

impl Printable for i32 {
  fn format(&self) -> String {
    format!("i32: {}", *self)
  }
}

impl Printable for String {
  fn format(&self) -> String {
    format!("string: {}", *self)
  }
}

fn print_it(z: &Printable) {
  println!("{}", z.format());
}

fn static_dispatch() {
  let a = 123;
  let b = "hello".to_string();

  // println!("{}", a.format());
  // println!("{}", b.format());

  print_it(&a); // this thing happens at run time. That's why it is dynamic.
  print_it(&b);
}
