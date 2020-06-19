// Funtions return a value
// Procedures do not return a value
#[allow(unused_variables)] // I don't have warnings about unused vars


fn main () {
  let returned_data = some_function(2.2, 50);
  println!("resturned_data is: {}", returned_data);
  some_str_procedure("test");

  let string_slice_var: &str = "Hello";
  some_str_procedure(string_slice_var);

  let string_var = String::from("I'm a real string");
  some_str_procedure(&string_var);

  some_string_procedure(string_var);
}

#[allow(dead_code)] // I don't have warnings about function don't use
fn some_function(param_a: f32, param_b: i128) -> f32 {
  println!("I'm in some_function!");
  // 10.1 // No semicolon means this is what's returned by the function
  if param_a < 100. {
    let return_var = 10.1 * param_a + param_b as f32;
    return_var
  } else {
    -1.
  }
}

#[allow(dead_code)] // I don't have warnings about function don't use
fn some_procedure(param_a: f32, param_b: i8) {
  println!("I'm in some_procedura with a {} b {}", param_a, param_b);
}

fn some_str_procedure(param: &str) {
  println!("I'm in some_str_procedure with param {}", param);
}

fn some_string_procedure(param: String) {
  println!("I'm in some_string_procedure with param {}", param);
}