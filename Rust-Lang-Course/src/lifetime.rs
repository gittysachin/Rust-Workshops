struct Person {
  name: String,
}

impl Person {
  fn get_ref_name(&self) -> &String {
    &self.name
  }
}

struct Company<'z> {
  // lets have the name of lifetime = z
  name: String,
  ceo: &'z Person,
}

fn lifetime() {
  // &'static str    // So essentially static is a lifetime, it's essentially a defenition of how long a variable will live
  // and static means that the variable is going to live as long as the program lives

  let boss = Person {
    name: String::from("Sachin Jangid"),
  };
  let vaiblabs = Company {
    name: String::from("VAIBLABS"),
    ceo: &boss,
  };

  // let mut z: &String;
  // {
  //   let p = Person {
  //     name: String::from("John"),
  //   };
  //   z = p.get_ref_name();      // it won't compile because of the lifetime issue
  //                                 it will saya that p does not live long enough
  // }
}
