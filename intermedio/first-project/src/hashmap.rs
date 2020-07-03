// El tipo HashMap<K, V> almacena
// un mapeo de claves de tipo K a valores de tipo V. Esto lo hace a través de una función de hashing,
// que determina cómo coloca estas claves y valores en la memoria.

use std::collections::HashMap;

fn main() {
    // Creando un HashMap vacío
    // Este HashMap tiene
    // claves de tipo String y valores de tipo i32. Al igual que los vectores, los mapas hash son
    // homogéneos: todas las claves deben tener el mismo tipo, y todos los valores deben tener el mismo
    // tipo
    let mut scores = HashMap::new();

    // Agregando datos al HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    // Otra forma de construir un mapa de hashmap es usando el método de agrupación en un vector de
    // tuplas, donde cada tupla consiste en una clave y su valor.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];

    // La notación de tipo HashMap<_, _> es necesaria porque es posible recolectar en muchas estructuras
    // de datos diferentes y Rust no sabe qué tipos de datos, a menos que se lo especifiques, van a
    // contener las claves y los valores.
    let scores2: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    // Ownsership
    // Para los tipos que implementan el trait Copy, como i32, los valores se copian en el mapa de hash.
    // Para valores propios como String, los valores se moverán y el mapa de hash será el propietario de
    // esos valores

    let field_name = String::from("Color favorito");
    let field_value = String::from("Azul");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // No podemos usar las variables field_name y field_value después de haberlas movido al mapa hash
    // con la llamada a insertar
    // Si insertamos referencias a valores en el mapa de hash, los valores no se moverán al mapa de hash.
    // Los valores a los que apuntan las referencias deben ser válidos al menos mientras el mapa de hash
    // sea válido

    let team_name = String::from("Blue");
    // El resultado está envuelto en Some porque get devuelve una Option<&V>; si no hay valor para esa
    // clave en el mapa de hash, get devuelve None
    let score_blue =  scores.get(&team_name);

    // Iterando
    // Este código imprimirá cada par en un orden arbitrario
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // Modificando el HashMap
    // Sobreescribiendo un valor
    // Si insertamos una clave y un valor en un hash map y luego insertamos esa misma clave con un valor
    // diferente, el valor asociado a esa clave será reemplazado
    scores.insert(String::from("Blue"), 25);

    // Insertar valor solamente si la clave no tenía un valor previo
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    // El valor de retorno del método entry es una enum llamado Entry que
    // representa un valor que puede o no existir
    // El método or_insert de entry está definido para devolver una referencia mutable al valor de la clave
    // Entry correspondiente si existe, y si no, inserta el parámetro como nuevo valor para esta clave y
    // devuelve una referencia mutable al nuevo valor
    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(30);
    println!("socres3 es: {:?}", scores3);

    // Actualizar un valor basándonos en un valor anterior
    let text = "Hello world wunderfull world";
    let mut map = HashMap::new();

    // Contando palabras
    for word in text.split_whitespace() {
        // El método or_insert devuelve una
        // referencia mutable (&mut V) al valor de esta clave. Aquí almacenamos esa referencia mutable en la
        // variable count, por lo que para asignar a ese valor debemos primero realizar la dereferencia
        // utilizando el asterisco (*)
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}