pub fn strings() {
  // let s = "hello there!";
  // utf-8 characters
  let s: &'static str = "hello there"; // &str = string slice

  for c in s.chars().rev() {
    println!("{}", c);
  }

  if let Some(first_char) = s.chars().nth(0) {
    println!("first letter is {}", fist_char);
  }

  // heap
  // String
  let mut letters = String::new();
  let mut a = 'a' as u8;
  while a <= ('z' as u8) {
    letters.push(a as char);
    letters.push(",");
    a = a + 1;
  }

  println!("{}", letters);

  let u: &str = &letters;

  // concatenation
  // String + str
  // let z = letters + &letters;

  let mut abc = "hello world".to_string();
  abc.remove(0);
  abc.push_str("!!!");
  println!("{}", abc.replace("ello", "goodbye"));
}
