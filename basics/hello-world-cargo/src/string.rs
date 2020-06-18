// String

fn main () {
  let mut my_string = String::from("My name is Byron");

  // Length
  println!("Length: {}", my_string.len());

  // Is empty
  println!("Is empty? {}", my_string.is_empty());

  for token in my_string.split_whitespace() {
    println!("{}", token);
  }

  println!("Does the string contain 'Byr'? {}", my_string.contains("Byr"));
  my_string.push_str(" Wellcome");
  println!("{}", my_string);
}