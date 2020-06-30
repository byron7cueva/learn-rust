// String Slice
// Es una referencia a una parte del string

fn main () {
    let s = String::from("Hello world");
    // Rango [indice_primero..longitud] sin incluir el final
    let hello = &s[0..5];

    // Rango [..=] Incluye el final
    let world = &s[6..=10];

    // Si el inicio del indice es 0, se lo puede obviar
    // let slice = &s[0..2];
    let slice = &s[..2];

    // Podemos obviar el ultimo indicando que vaya hasta el final del string
    let len = s.len();
    //let slice_final = &s[3..len];
    let slice_final = &s[3..];

    // Se puede eliminar el inicio  el final del indice
    // siempre y cuando el inicio y final coincidan con el inicio y final del string
    let slice_igual = [..];

    let mut cadena = String::from("Hello world");
    let first = first_word(&cadena);
    // Clear intenta tomar una referencia mutable
    // Por lo tanto genera un error ya que como regla si tenemos una referencia inmutable no podemos tomar una inmutable
    // cadena.clear(); // Ya se limpia la cadena, esto genera un error ya que fist guarda una referencia
    println!("La primera palabra es: {}", first);

    //Slice de un array
    let a = [0, 1, 2, 3, 4];
    //[indice_primero..longitud] [..] exclusive , [..=] inclusive
    let slice = &a[1..=3];
    println!("El slice es {:?}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}