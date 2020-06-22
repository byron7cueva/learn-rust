#[allow(unused_variables)]

fn main () {
  let some_bool = true;
  let some_int = 30;
  let some_int2 = 50;

  if some_bool == true || (some_int > 100 && some_int2 == 200) {
    println!("It is true");
  } else if some_int == 30 {
    println!("Else if");
  } else {
    println!("It is false");
  }

  // No use semicolon for return value
  let var_from_inline = if some_int == 9 { 300 } else { 400 };

  let var_from_inline2 = if some_int == 9 { 
    300 // return this value
  } else if some_int2 == -3 {
    println!("Test");
    0
  } else {
    400
  };

  match some_bool {
    true => {
      println!("Its true");
    }
    false => {
      println!("Its false");
    }
  }

  match some_int {
    0 => println!("Hit 0 brach"),
    1..=100 => println!("Between 1 and 100 branch"),
    _ => println!("Else branch")
  }

  let var_from_match = match some_bool { true => 10, false => 20 };

  let var_from_match2 = match some_int {
    0 => 0,
    1 | 2 => 100,
    _ => 200
  };
}