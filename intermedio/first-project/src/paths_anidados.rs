// Paths anidados
// Podemos usar rutas anidadas para llevar los mismos elementos al ámbito de aplicación en una línea
// en lugar de dos, especificando la parte común de la ruta
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// Desdoblar paths
// Para desdoblar estas dos rutas en una sola sentencia use, podemos usar self en la ruta anidada
// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// Si deseas incluir todos los elementos públicos definidos en una ruta al ámbito de aplicación, puedes
// utilizar especificar esa ruta seguida por *, el operador glob:
use std::collections::*;

fn main () {

}