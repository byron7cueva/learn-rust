// Memory mangament en Rust
// Ownership: Values owned by creator
// El creador es el dueño de los datos
// Values moved via assignment
// Los valores son movidos cada vez que se asignan
// When final owner returns, value is freed.
// Cuando el dueño de esa memoria se va del scope, los recursos que tiene asociados son liberados
// Entonces el creador de la memoria es quien la destruye, no hay la posibilidad de que exista double free errors
// no hay la posibilidad que exista memory leaks, siempre se liberan los recursos cuando se sale del scope
// Borrowing: 
// Es un prestamo de una referencia a la memoria de solo lectura
// Mutable Borrowing 
// En este caso se presta una referencia pera a la vez se le permite modificar
// Lifetime of a borrow
// Lifetime: Es el tramo de código donde se usa la referencia
// Cuando se realiza un borrow (prestando esa variable), el compilador hace un analisis y se fija en el codigo
// durante el cual se esta prestando la referencia
// Inpone ciertas reglas
// 1. Si hay una referencia compartida, no hay escritores durante la vigencia del préstamo compartido. Congela los datos temporalmente.
// 2. Si hay una referencia mutable, no se permite ningun otro lector o escritor durante la vida útil del préstamo mutable.
//    Solo se puede acceder a la memoria a traves de ese borrow mutable.

#[derive(Debug)]
struct Apple {}

impl Apple {
  fn new() -> Self {
    Self {}
  }
}

fn main () {
  // Ownership
  let apple = Apple::new();
  // Cuando se hace una asignacion o llamada a una funcion (asignacion implicita) se transfiera la propiedad del valor
  eat(apple); // La funcion se vuelve dueña del valor
  // eat(apple); // Error ya que el propietario del valor fue movido

  // Borrowing
  let apple2 = Apple::new();
  let mut bag = Vec::new();
  bag.push(apple2);
  bag.push(Apple::new());
  let weight = weigh(&bag); // Se presta la bag a la funcion
  // En este caso el encargado de liberar la memoria sigue siendo bag que el dueño
  println!("Bag {:?} weights {}", bag, weight);

  // Mutable Borrowing
  let apple3 = Apple::new();
  let mut bag2 = Vec::new();
  bag2.push(apple3);
  bag2.push(Apple::new());
  change(&mut bag2); // En este caso se presta pero se permite modificar
  // Aqui bag2 sigue siendo el dueño y el encargado de liberar de memoria
  println!("Bag is now {:?}", bag2);

  // Peligros de la mutación
  let mut buffer = format!("Hello");
  let slice = &buffer[1..];
  // Al hacer el push crea memoria nueva para reservar la capacidad necesaria para almacenar a los elementos
  // Borra la memoria anterior que tenia asignado
  // Entonces slice sufre un Daging reference, ya que esta apuntando a un espacio de memoria que ya no existe
  // buffer.push_str("world"); // Por lo tanto esto rust no permite y genera un error de compilacion, antes de
  // generar un problema de seguridad
  println!("{:?}", slice);

  // Borrow inmutable
  let mut buffer2 = format!("Hello");
  let slice2 = &buffer2; // Se realiza un prestamo inmutable
  // buffer2.push_str(" world"); // Error - No se puede mutar mientras esta compartido
  // slice2.push_str(" world"); // Error - No se puede mutar a traves de una referencia inmutable
  println!("{:?}", slice2); // Se puede leer de slice2 mientras este compartido
  buffer.push_str(" world"); // Despues del ultimo uso de slice2, buffer2 es mutable otra vez

  // Mutable Borrow
  let mut buffer3 = format!("Hello");
  buffer3.push_str(" world"); // Buffer es mutable
  let slice3 = &mut buffer3; // Creando referencia mutable en slice3
  // println!("{:?}", buffer3); // Error - No se puede acceder a buffer3 mientras exista un borrow mutable
  slice3.push_str(" world"); // Pero puede mutarse a traves de slice3
  buffer3.push_str(" world"); // Despues del ultimo uso de slice3, buffer es accesible
}

// La variable apple_param de la funcion se vuelve dueña del valor pasado
// La funcion pasa aa ser la encargada de liberar el espacio de memoria cuando termine
fn eat (apple_param: Apple) {

}

// La funcion toma prestada una referrencia al vector bag
// Solo puede usar sin modificar
fn weigh(bag: &Vec<Apple>) -> usize {
  bag.len()
}

fn change(bag: &mut Vec<Apple>) {}