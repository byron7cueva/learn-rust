pub fn test_stack() {
    // Ya que los tipos escalares, array y tuplas (Si todos sus tipos son Copy, escalares) no cambian su espacio de necesidad de memoria
    // Los escalares (tipo entero, booleano, coma flotante, char) implementan el trait Copy
    // Esto se almacenan en el stack y no requieren del concepto de ownership
    // Estos se quitan de la pila cuando su alcance ha sido terminado
    let num = 12;
    let num2 = num;
    println!("El numero es {}", num);

    let array = [1, 2, 3, 4];
    let array2 = array;
    println!("EL array es {:?}", array);

    // Esta es un literal de cadena es inmutable
    // Este se almacena en el stack
    // Su contenido se codifica directamente en el ejecutable final
    let literal_cadena: &str = "Hola";
    let literal_cadena_b = literal_cadena;
    println!("La cadena es: {}", literal_cadena);
}

pub fn test_heap() {
    // Ownership
    // Aplica a String, otros tipos de datos complejos de la libreria estandar y creados por el usuario

    // Tipo String
    // Se almacena en el Heap
    // Es capaz de almacenar una cantidad de texto que es desconocida en tiempo de compilación
    // Creando un string a partir de un literal de cadena
    // Se solicita un espacio de memoria en tiempo de ejecucion para podelo crear
    let mut cadena = String::from("Hola");
    cadena.push_str(" mundo");
    println!("{}", cadena);

    // Move (Movimiento)
    // Rust nunca creara automaticamente copias profundas
    // Se puede suponer que cualquier copia automatica es economica en terminos de rendimiento
    let s = String::from("Hola");
    // Se mueve la propiedad a s2 de la variable s
    // s queda como invalido y s2 como valida
    // Cuando salga del ambito solo se liberara s2
    let s2 = s;
    // println!("{}", s); // Esto da error

    // Copia profunda
    // Se copian los datos de la pila y los datos del heap de la cadena
    // Clone esto resultara caro
    let string_1 = String::from("Hello");
    let string_2 = string_1.clone();
    println!("s1 = {}, s2 = {}", string_1, string_2);
}

