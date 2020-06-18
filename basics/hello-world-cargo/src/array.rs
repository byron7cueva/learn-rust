// Array

fn main () {
  let numbers = [1, 2, 3, 4, 5];

  for n in numbers.iter() {
    println!("{}", n);
  }

  for i in 0..numbers.len() {
    println!("{}", numbers[i]);
  }

  // Definiendo un array de enteros de 5 elementos
  let numbers2: [i32; 5] = [1, 2, 3, 4, 5];

  for n in numbers2.iter() {
    println!("{}", n);
  }

  // Creando un array de 100 elementos que en cada posicion tenga 2 como valor
  let numbers3 = [2; 100];

  for n in numbers3.iter() {
    println!("{}", n);
  }

}