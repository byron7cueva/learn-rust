// Uso de límites de traits para implementar métodos de forma condicionada

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Mediante el uso de un trait vinculado a un bloque que utiliza parámetros de tipo genérico, podemos
// implementar métodos condicionados para los tipos que implementan los rasgos especificados.
// Pero Pair<T> sólo implementa el método cmp_display si su tipo interno T implementa el rasgo
// PartialOrd que permite la comparación y el rasgo Display que permite la impresión
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("El mayor es x = {}", self.x);
        } else {
            println!("El mayor es y = {}", self.y);
        }
    }
}