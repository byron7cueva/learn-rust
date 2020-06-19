// Regular Expresion
use regex::Regex;

fn main () {
  // let re = Regex::new(r"\d");
  // let re = Regex::new(r"\w{5}").unwrap();
  let re = Regex::new(r"(\w{5})").unwrap();
  let text = "byron";

  println!("Found match? {}", re.is_match(text));

  match re.captures(text) {
    // Some(caps) => println!("Find mach: {}", caps.get(0).unwrap().as_str()),
    Some(caps) => println!("Find mach: {}", &caps[0]),
    None => println!("Could not find match")
  }
}