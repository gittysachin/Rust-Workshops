fn use_slice(slice: &mut [i32]) {
  println!("first elem = {}, len = {}", silce[0], slice.len());
  slice[0] = 4321;
}

pub fn slices() {
  // it's size is not known at compile time
  let mut data = [1, 2, 3, 4, 5];
  // use_slice(&mut data[1..4]);
  use_slice(&mut data);
  println!("{:?}", data);
}
