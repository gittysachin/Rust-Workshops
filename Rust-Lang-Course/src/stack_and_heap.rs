// You can not allocate infinitely large array on the stack. It has certain limitations.

// The other choice of allocation is dynamic allocation and it is called heap.
// WE use box type for allocating heap

#![allow(dead_code)]

use std::mem;

struct Point {
  x: f64,
  y: f64,
}

fn origin() -> Point {
  Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
  let p1 = origin();
  let p2 = Box::new(origin()); // Store the address where memory is actually kept and the address takes up 64 bits.
                               // In this location we're locating to the heap

  println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // both f64 (x and y) combined and it takes total pf 16 bytes
  println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // Here the box is pointing to origin. Because of the 64 bit address, we git 8 bytes size

  let p3 = *p2;
  println!("{}", p3.x); // In this location we're locating back to the stack
}
