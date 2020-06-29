// loop
fn main () {
  let mut n = 0;

  // Ejecuta el codigo una y otra vez para siempre o hasta que se le diga explicitamente que pare
  loop {
    n += 1;

    if n == 7 {
      // Para escapar que complete la iteracion si continue con la siguiente
      continue;
    }

    if n > 10 {
      // Indica cuando debe salir del bucle
      break;
    }

    println!("The n is {}", n);
  }

  // Devolviendo un resultado
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      // Retornando el valor
      break counter * 2;
    }
  };
  assert_eq!(result, 20);
}