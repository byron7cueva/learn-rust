// Iteradores
// Existen tres métodos que se pueden utilizar en la colección al trabajar con for: iter, into_iter e
// iter_mut

fn main () {
    let names = vec!["Luis", "Pedro", "Carlos"];

    // iter()
    // Esto toma prestado cada elemento de la colección a través de cada iteración. De este modo,
    // la colección queda intacta y disponible para su reutilización después del bucle.
    for name in names.iter() {
        match name {
            &"Pedro" => println!("Eres {} felicidades", name),
            _ => println!("El nombre es {}", name)
        }
    }
    println!("{}", names[0]); // Esto es permitido

    // into_iter()
    // Esto consume la colección para que en cada iteración se proporcionen los datos
    // exactos. Una vez que la colección ha sido consumida, ya no está disponible para su reutilización, ya
    // que ha sido `movida' dentro del bucle.
    for name in names.into_iter() {
        match name {
            "Carlos" => println!("Felicidades eres {}", name),
            _ => println!("El nombre es {}", name)
        }
    }

    // println!("{}", names[0]); // Esto no es permitido y dara un error ya que el valor de names fue movido

    // iter_mut()
    // Toma prestado cada elemento de la colección de manera mutable, permitiendo que la
    // colección sea modificada en el bucle.
    let mut numbers = vec![1, 2, 3, 4];
    for i in numbers.iter_mut() {
        match i {
            1 => *i = 88,
            _ => println!("No es el 1")
        }
    }
    println!("{}", numbers[0]); // Esto devuelve 88, ya que se cambio su valor

    // Iterando pasando la referencia
    let names_2 = vec!["Marco", "Oscar", "Elvis"];
    for name in &names_2 {
        match name {
            &"Oscar" => println!("Eres {} felicidades", name),
            _ => println!("Hola {}", name)
        }
    }
}