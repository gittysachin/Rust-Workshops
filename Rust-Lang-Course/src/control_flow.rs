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

fn control_flow() {
  if_statement();
}
