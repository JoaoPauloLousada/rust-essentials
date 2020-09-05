pub fn run() {
  // Print to console
  println!("Hello World from the print.rs file!");

  // Template literals
  println!("Number {}", 1);
  
  // Basic formatting
  println!("{} is from {}", "Brad", "Mass");
  
  // PositionalArguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug traits
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}