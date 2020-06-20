use std::mem;

fn array() {
  let mut a: [i32; 5] = [1, 2, 3, 4, 5];

  println!("a has {} elements, first is {}", a.len(), a[0]);
  a[0] = 321;
  println!("a[0] = {}", a[0]);

  println!("{:?}", a);

  if a == [321, 2, 3, 4, 5] {
    println!("match");
  }

  let b = [1; 10]; // let b:[u8;10]    or   let b = [1u16;10];
                   // b.len() == 10   and all the elements are 1
  for i in 0..b.len() {
    println!("{}", b[i]);
  }

  println!("b took up {} bytes", mem::size_of_val(&b));

  let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 3.0]];

  println!("{:?}", mtx);

  for i in 0..mtx.len() {
    for y in 0..mtx[i].len() {
      if i == j {
        println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
      }
    }
  }
}