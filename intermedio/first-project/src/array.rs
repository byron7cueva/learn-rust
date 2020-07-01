use std::mem;

// Array
// Un array es una colección de objetos del mismo tipo T, almacenados en memoria contigua. Los
// arrays se crean usando corchetes [], y su tamaño, que se conoce en tiempo de compilación, es parte
// de su definición de tipo[T; tamaño]

fn main () {
    // Array fijo
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // Inicializando los elementos a un mismo valor, en este caso cero
    let ys: [i32; 500] = [0; 500];
    println!("Obteniendo la primera posicion de xs = {}", xs[0]);
    println!("Obteniendo la segunda posicion de xs = {}", xs[1]);
    println!("La longitud de xs = {}", xs.len());

    // Las matrices se asignan en el stack
    println!("El array xs ocupa {} bytes", mem::size_of_val(&xs));
}