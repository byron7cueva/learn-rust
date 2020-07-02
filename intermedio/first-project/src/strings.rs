// Strings
// Los Strings son UTF-8, por lo que podemos incluir cualquier dato correctamente codificado en
// ellos

#[allow(unused_variables)] // I don't have warnings about unused vars
#[allow(unused_assignments)] // I don't have warnings about unused assignments

fn main () {
  // &str is more complex
  // Immutable
  // Often allocated on the stack, sometime a heap references, sometime embedded in the code
  let example_str: &str = "Hello";

  // String
  let mut s = String::new(); // Creando un string vacio
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
  // push_str añade una slice al final del String
  mut_string.push_str(example_str);
  mut_string.push_str("Some harcode literal");
  mut_string.push('m');
  mut_string.push('e');
  // El método push_str toma una slice porque no
  // necesariamente queremos tomar posesión de la cadena
  let s2 = "hola";
  mut_string.push_str(s2);
  //El método push toma un solo carácter como parámetro y lo añade a la cadena
  mut_string.push('p');

  println!("mut_string: {}", mut_string);

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

  // Se puede utilizar el operador + o la macro format! para concatenar los valores de String
  let s3 = String::from("Hello, ");
  let s4 = String::from("world");
  // La razón por la que s3 ya no
  // es válida después de la operación de adición y la razón por la que usamos una referencia a s4 tiene
  // que ver con la definición del método al que se llama cuando usamos el operador +. El operador +
  // utiliza el método add, cuya definición es parecida a esta:
  // fn add(self, s: &str) -> String {
  let s5 = s3 + &s4;

  // Para combinaciones de cadenas más complicadas, podemos utilizar la macro format!
  let st1 = String::from("tic");
  let st2 = String::from("tac");
  let st3 = String::from("toe");
  let st = format!("{} - {} - {}", st1, st2, st3);
}