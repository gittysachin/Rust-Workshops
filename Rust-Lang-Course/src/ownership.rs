fn ownership() {
  let v = vec![1, 2, 3]; // variable v kind of owns the memory that is stored in the vector

  let v2 = v;

  let foo = |v: Vec<i32>| ();
  foo(v);

  println!("{:?}", v);

  let u = Box::new(1); // i32 Copy 
  let u2 = u;

  // println!("u = {}", *u);

  let print_vector = |x:Vec<i32>| -> Vec<i32> {
    println!("{:?}", x);
    x
  }
  let vv = print_vector(v);
  println!("{}", vv[0]);
}
