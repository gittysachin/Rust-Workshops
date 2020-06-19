use std::mem;

const MEANING_OF_LIFE: u8 = 42; // no fixed address

static mut z: i32 = 123; // This one is yet another option to have a global value

fn using_constants() {
  unsafe {
    z = 777;
    println!("{}", z);
  }
}

// So if you're to use constants then, const is a much better option instead of using static, because with static -- the security might be compromised.
