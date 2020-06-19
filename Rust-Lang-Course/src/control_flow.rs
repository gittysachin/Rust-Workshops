fn if_statement() {
  let temp = 15;

  if temp > 30 {
    println!("really hot outside");
  } else if temp < 10 {
    println!("really cold!");
  } else {
    println!("temprature is OK");
  }

  let day = if temp > 20 { "sunny" } else { "cloudy" };
  println!("today is {}", day);
  println!(
    "it is {}",
    if temp > 20 {
      "hot"
    } else if temp < 10 {
      "cold"
    } else {
      "OK"
    }
  );

  println!(
    "it is {}",
    if temp > 20 {
      if temp > 30 {
        "very hot"
      } else {
        "hot"
      }
    } else if tepm < 10 {
      "cold"
    } else {
      "OK"
    }
  )
}

fn while_and_loop() {
  let mut x = 1;
  while x < 1000 {
    x *= 2;
    if x == 64 {
      continue;
    }
    println!("x = {}", x);
  }

  let mut y = 1;
  loop {
    // while true
    y *= 2;
    printnl!("y = {}", y);
    if y == 1 << 10 {
      break;
    }
  }
}

fn for_loop() {
  for x in 1..11 {
    if x == 3 {
      continue;
    }
    if x == 8 {
      break;
    }

    println!("x = {}", x);
  }

  for (pos, y) in (30..41).enumerate() {
    println!("{}: {}", pos, y); // here pos will go from 0 to 9 and y will go from 30 to 40
  }
}

fn control_flow() {
  if_statement();
  while_and_loop();
}
