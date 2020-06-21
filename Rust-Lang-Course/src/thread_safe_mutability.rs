use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
  name: Arc<String>,
  state: Arc<Mutex<String>>,
}

impl Person {
  fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
    Person {
      name: name,
      state: state,
    }
  }

  fn greet(&self) {
    // self.state.clear();
    // self.state.push("excited");
    let mut state = self.state.lock().unwrap();
    state.clear();
    state.push_str("excited");

    println!("Hi, my name is {} and I am {}", self.name, state.as_str());
  }
}

fn Mutex_demo() {
  let name = Arc::new("Sachin".to_string());
  let state = Arc::new(Mutex::new("bored".to_string()));
  let person = Person::new(name.clone());

  let t = thread::spawn(move || {
    person.greet();
  });

  println!(
    "Name = {}, state = {}",
    name,
    state.lock().unwrap().as_str()
  );

  t.join().unwrap();
}
