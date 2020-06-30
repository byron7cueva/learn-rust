// Tuple Struct
// Las tuple struct no tienen nombres asociados con sus campos
// solo tienen los tipos de los campos
// Las tuple struct son útiles cuando se quiere dar
// un nombre a toda la tupla y hacer que la tupla sea de un tipo diferente a otras tuplas

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Estructuras Unit-Like son similar a ()
// pueden ser útiles en situaciones en las que se necesita implementar un rasgo en algún tipo pero
// no se dispone de ningún dato que se desee almacenar
struct UniLike();

fn main () {
    let back = Color(0, 0, 0);
    let origin = Point(0, 0 ,0);
}