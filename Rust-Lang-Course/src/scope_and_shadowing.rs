use std::mem;

fn scope_and_shadowing() {
  let a = 123;
  let a = 1234;

  {
    let b = 456;
    println!("b = {}", b);

    let a = 777;
    println!("inside, a = {}", a);
  }

  println!("outside, a = {}", a);

  // println!("outside, b = {}", b);
}
