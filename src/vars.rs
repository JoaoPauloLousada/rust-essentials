// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is blocked-scoped language

pub fn run() {
  let name = "Brad";
  
  // the word mut is to tell the compiler that this variable is mutable
  let mut age = 37;
  println!("My name is {} and I am {}", name, age);

  age = 38;
  println!("My name is {} and I am {}", name, age);

  // Define contants
  const ID: i32 = 001;
  println!("id: {}", ID);

  // Assign multiple variables
  let (my_name, my_age) = ("Brad", 37);
  println!("{} is {}", my_name, my_age);
}