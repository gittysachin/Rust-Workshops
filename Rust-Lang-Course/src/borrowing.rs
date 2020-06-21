fn borrow() {
  let print_vector = |x:&Vec<i32>| {
    println!("x[0] = {}", x[0]);
    // x.push(123);
  }

  let v = vec![3,2,1];
  print_vector(&v);
  println!("v[0] = {}", v[0]);

  let mut a = 40;
  {
    let b = &mut a;
    *b += 2;
  }

  println!("a = {}", a);
  
  let mut z = vec![3,2,1];

  for i in &z {
    println!("i = {}", i);
    // z.push(5);    // rust prevents us from doing this
    // thereby it enforces some memory safety. 
  } 
}
