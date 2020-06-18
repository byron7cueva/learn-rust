// Defining Traits

// Create the struct Person
struct Person {
  name: String,
  age: u8
}

// Create contract o interface
trait HasVoiceBox {
  // Speak
  fn speak(&self);
  // Check if can speak
  fn can_speak(&self) -> bool;
}

// Implements the trait
impl HasVoiceBox for Person {

  fn speak(&self) {
    println!("Hello my name is {}", self.name);
  }

  fn can_speak(&self) -> bool {
    if self.age > 1 {
      return true;
    }
    return false;
  }
}

fn main () {
  let person = Person {
    name: String::from("Byron"),
    age: 30
  };

  println!("Can {} speak? {}", person.name, person.can_speak());
}