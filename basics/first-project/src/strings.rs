// Strings
#[allow(unused_variables)] // I don't have warnings about unused vars
#[allow(unused_assignments)] // I don't have warnings about unused assignments

fn main () {
  // &str is more complex
  // Immutable
  // Often allocated on the stack, sometime a heap references, sometime embedded in the code
  let example_str: &str = "Hello";

  // String -> Heap and Mutable
  let example_string: String = String::from("Hello");

  let string_from_str: String = example_str.to_string();
  let string_from_str2: String = "Some hardcode string".to_string();


  let string_from_harcoded = String::from("Some harcoded");
  let string_from_str_var = String::from(example_str);

  // Conversion directa
  let str_from_string: &str = &example_string;

  // Combined strings
  let combined_string_literals = ["first", "second"].concat();
  let combined_with_format_macro = format!("{} {}", "first", "second");
  // let string_plus_str = example_string + example_str;
  let mut mut_string = String::new();
  mut_string.push_str(example_str);
  mut_string.push_str("Some harcode literal");
  mut_string.push('m');
  mut_string.push('e');

  let a = String::from("a");
  let b = String::from("b");
  let combined = a + &b + &mut_string;

  let str_from_substring: &str = &example_str[0..2];
  let str_from_substring2: &str = &example_str[0..=2]; // Include de caracter un position 2

  // Get char of index
  let char_by_index = &example_str.chars().nth(1);

  match char_by_index {
    Some(c) => println!("Found a char {}", c),
    None => {}
  }

  if let Some(c) = example_str.chars().nth(1) {
    println!("Found a char {}", c);
  }

}