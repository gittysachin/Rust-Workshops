use std::rc::Rc;

struct Person {
  name: Rc<String>,
}

impl Person {
  fn new(name: Rc<String>) -> Person {
    Person { name: name }
  }

  fn greet(&self) {
    println!("Hi, my name is {}", self.name);
  }
}

fn rc_demo() {
  let name = Rc::new("Sachin".to_string());
  println!(
    "Name = {}, name has {} many pointers",
    name,
    Rc::strong_count(&name)
  );
  {
    let person = Person::new(name.clone());

    person.greet();
    println!(
      "Name = {}, name has {} many pointers",
      name,
      Rc::strong_count(&name)
    ); // this isn't going to work if we don't use Rc
  }
  println!(
    "Name = {}, name has {} many pointers",
    name,
    Rc::strong_count(&name)
  );
}
