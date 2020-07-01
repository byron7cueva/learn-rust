// slice
// Son similares a los arrays, pero su tamaño no se conoce en el momento de la compilación.
// En cambio, una slice es un objeto de dos palabras, la primera palabra es un puntero a
// los datos, y la segunda palabra es la longitud de la slice. El tamaño de la palabra es el mismo que el
// de usize, determinado por la arquitectura del procesador, por ejemplo 64 bits en un x86-64. Las
// slices se pueden usar para tomar prestada una sección de un array, y tienen el tipo &[T].

fn main () {
  let xs: [i32; 5] = [1, 2, 3, 4, 5];
  // Los arrays pueden ser prestados (borrow) como un slice
  analyze_slice(&xs);
  // Los slice pueden apuntar a una seccion del array
  analyze_slice(&xs[1..4]); // Esclusive la posición 4, es deci va tomar desde la posición 1 hasta 3
}

fn analyze_slice(slice: &[i32]) {
  println!("{:?}", slice);
  println!("El primer elemento del slice es {}", slice[0]);
  println!("La longitud del slice es {}", slice.len());
}