pub fn vectors() {
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);

  println!("a = {:?}", a);

  a.push(44);

  println!("a = {:?}", a);

  // usize isize

  let idx: usize = 0;

  a[idx] = 312;
  println!("a[0] = {}", a[idx]);

  // Option
  match a.get(6) {
    Some(x) => println!("a[6] = {}", x),
    None => println!("error, no such element"),
  }

  for x in &a {
    println!("{}", x);
  }

  a.push(77);

  println!("{:?}", a);

  let last_elem = a.pop(); // Option

  println!("last element = {:?}, a = {:?}", last_elem, a); // last element = Some(77), a = [312, 2, 3, 44]

  while let Some(x) = a.pop() {
    println!("{}", x);
  }
}
