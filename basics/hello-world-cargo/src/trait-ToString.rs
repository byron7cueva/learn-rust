// trait
// This is similar to interface in Java

struct Person {
  name: String,
  age: u8
}

// ToString is trait
impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("My name is {} and I am {}.", self.name, self.age);
  }
}

fn main () {
  let byron = Person {
    name: String::from("Byron"),
    age: 30
  };

  println!("{}", byron.to_string());
}