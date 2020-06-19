use serde_json::Value as JsonValue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
  name: String,
  age: u8,
  is_male: bool
}

fn main () {
  let json_str = r#"
    {
      "name": "Byron",
      "age": 30,
      "is_male": true
    }
  "#;

  let res = serde_json::from_str(json_str);

  if res.is_ok() {
    // let p: JsonValue = res.unwrap();
    // println!("The name is {}", p["name"]);
    // println!("The name is {}", p["name"].as_str().unwrap());
    let p: Person = res.unwrap();
    println!("The name is {}", p.name);
    println!("The age is {}", p.age);
    println!("Is male {}", p.is_male);
  } else {
    println!("Sorry! Could not pase JSON");
  }
}