

fn main () {
  let name = String::from("Byron");

  println!("Character at index 8: {}", match name.chars().nth(1){
    Some(c) => c.to_string(),
    None => "No character at index 8!".to_string()
  });


  println!("Occupation is {}", match get_occupation("Carlos") {
    Some(o) => o,
    None => "No ocupation found"
  });
}

fn get_occupation(name: &str) -> Option<&str> {
  match name {
    "Byron" => Some("Software Developer"),
    "Luis" => Some("Dentist"),
    _ => None
  }
}