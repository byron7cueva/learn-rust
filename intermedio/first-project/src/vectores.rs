// Vectores
// Los vectores te
// permiten almacenar más de un valor en una sola estructura de datos que pone todos los valores uno
// al lado del otro en la memoria. Los vectores sólo pueden almacenar valores del mismo tipo.

fn main () {
    // Crear un nuevo vector vacío
    // Observa que hemos añadido una anotación de tipo. Debido a que no estamos insertando ningún
    // valor en este vector, Rust no sabe qué tipo de elementos intentamos almacenar. Este es un punto
    // importante.
    let vect1: Vec<i32> = Vec::new();

    // Rust puede inferir el tipo de valor que desea almacenar una vez que inserta los valores
    // Rust proporciona la macro vec! esta creará un nuevo vector que contiene los valores que le das
    let vect2 = vec![1, 2, 3];

    let mut vect3: Vec<i32> = Vec::new();
    vect3.push(1);
    vect3.push(2);
    vect3.push(3);
    vect3.push(4);

    let thrird: &i32 = &vect3[2];
    println!("El tercer elemento es {}", thrird);

    match vect3.get(2) {
        Some(t) => println!("El tercer elemento es {}", t),
        None => println!("No tiene un tercer elemento")
    }

    let mut vect4 = vec![1, 2, 3, 4];
    let first = &vect4[0];
    // Esto dara un error
    // Ya que no se puede tener una referencia inmutable y mutable en el mismo ambito
    // vect4.push(5);
    println!("El primer elemento es: {}", first);
    // Usar un bucle para obtener referencias inmutables a cada elemento en un vector
    for i in &vect4 {
        println!("{}", i);
    }

    // También podemos iterar sobre las referencias mutables de cada elemento en un vector mutable para
    // hacer cambios en todos los elementos
    for i in &mut vect4 {
        // Para cambiar el valor al que se refiere la referencia mutable, tenemos que utilizar el operador de
        // referencia (*) para llegar al valor en i antes de poder utilizar el operador +=
        *i += 50;
    }

    let number_list = vec![34,50, 25, 100, 65];
    let result = largest(&number_list);
    println!("El núemero mayor de la lista es {}", result);
    assert_eq!(result, 100);
}

// Encontrar el número más grande de una lista de números
// list representa cualquier slice de i32 que podamos pasar a la función
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}