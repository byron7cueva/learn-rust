// Vectores
// Los vectores sólo pueden almacenar valores del mismo tipo
// Pero un enum puede definir, bajo el mismo tipo, distintos “subtipos”, así que cuando
// necesitamos almacenar elementos de un tipo diferente en un vector, podemos definir y usar un
// enum

// Podemos definir un enum cuyas variantes contendrán los diferentes tipos de valores, y
// entonces todas las variantes del enum se considerarán del mismo tipo: el del enum. Podemos crear
// un vector de ese enum y así, finalmente, contenga diferentes tipos
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main () {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hola")),
        SpreadsheetCell::Float(10.2)
    ];
}