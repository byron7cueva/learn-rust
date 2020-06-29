/*
Estructura de control if and else
*/
fn main () {
  let n = 20;
  
  if n == 45 {
    println!("The number is equals to 45");
  } else if n > 50 {
    println!("The number is grate to 50");
  } else {
    println!("The number is diferent to 45 and less that 50");
  }

  // Usando if en una declaración let
  // El valor de la variable number estara vinculado al valor basado en el resultado de la expresión if
  // Los valores que tienen el potencial de ser resultados de cada rama del if deben ser del mismo tipo
  // Si los tipos no coinciden tendremos un error
  let condition = true;
  let number = if condition {
    5
  } else {
    6
  };
  println!("El valor de number es: {}", number);
}