// traits -> Rasgos (Similares a las interfaces en otros lenguajes de programacion)
// Definimos un trait usando la palabra clave trait y luego el nombre del rasgo
// Dentro de las llaves declaramos los métodos que describen los comportamientos de los
// tipos que tienen este rasgo.

// Cada tipo que implemente este rasgo debe proporcionar su propio
// comportamiento personalizado para el cuerpo del método

// Una restricción a tener en cuenta con las implementaciones de traits es que podemos implementar
// un trait en un tipo sólo si el rasgo o el tipo es local a nuestra caja
// no podemos implementar rasgos externos en tipos externos. Por ejemplo, no podemos
// implementar el rasgo Display en Vec<T> dentro de nuestra caja de agregadores, porque Display y
// Vec<T> están definidos en la biblioteca estándar y no son locales a nuestra caja

use std::fmt::{Display, Debug};

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Implementaciones por defecto
trait Summary2 {
    fn summarize(&self) -> String {
        String::from("Contenido generico")
    }
}

struct OtherContent {
    author: String,
    content: String
}

// Para utilizar una implementación predeterminada en lugar
// de definir una implementación personalizada, especificamos un bloque impl vacío
impl Summary2 for OtherContent {}

// Las implementaciones predeterminadas pueden llamar a otros métodos del mismo trait, incluso si
// esos otros métodos no tienen una implementación predeterminada
// No es posible llamar a la implementación predeterminada desde una
// implementación sobreescrita de ese mismo método.
trait Summaray3 {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Leer mas de {}", self.summarize_author())
    }
}

struct Tweet2 {
    username: String,
    content: String
}

impl Summaray3 for Tweet2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("Byron"),
        content: String::from("Este es un contenido"),
        reply: false,
        retweet: false
    };

    println!("Un nuevo tweet: {}", tweet.summarize());

    let other_content = OtherContent{
        author: String::from("Byron"),
        content: String::from("Este es un contenido")
    };

    println!("El resumen es: {}", other_content.summarize());

    let tweet2 = Tweet2 {
        username: String::from("Byron"),
        content: String::from("Ejemplo de contenido")
    };

    println!("El resuen del tweet2 es: {}", tweet2.summarize());

    notify(tweet);
}

// Traits como argumentos
// Podremos utilizar como parámetro cualquier tipo que tenga implementado el
// trait Sumary
fn notify(item: impl Summary) {
    println!("El resumen es: {}", item.summarize());
}

// Límites de los traits
// Utilizar impl para indicar que los parámetros de una función deben tener implementado el trait
// correspondiente es útil cuando únicamente hay un parámetro como en el caso anterior. Pero existe
// otra forma:
fn notify2<T: Summary>(item: T) {
    println!("El resumen es: {}", item.summarize());
}

// impl Trait es mejor para pocos
// parámetros, la segunda forma cuando hay más parámetros que contienen ese trait
// Esto funcionará bien si se permitiera que item1 e item 2 tuvieran tipos diferentes (siempre y cuando
// ambos implementen Summary)
fn notify3(item1: impl Summary, item2: impl Summary) {

}

// Si quisieras forzar a ambos a tener exactamente el mismo tipo
fn notify4<T: Summary>(item1: T, item2: T) {

}

// Limitando a varios traits
// necesita implementar dos traits diferentes al mismo tiempo
// Podemos indicarlo en la definición de la función con +
fn notify5(item: impl Summary + Display) {

}
// O
fn notify6<T: Summary + Display>(item: T) {

}

// Sintaxis alternativa para especificar los límites de los traits dentro de una cláusula where
// después de la definición de la función
// Usando +
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {
}

// Usando where
fn some_function2<T,U>(t: T, u: U)
    where T: Display + Clone,
          U: Clone + Debug {

}

// Traits devueltos cómo resultado de una función
// Podemos usar la sintaxis “impl Trait” en la posición de retorno de la función para devolver algo que
// implemente un trait
fn return_summarizable() -> impl Summary {
    // Esta definición dice: "Voy a devolver algo que implementa el trait Summary, pero no voy a decirte
    // el tipo exacto". En nuestro caso, estamos devolviendo un Tweet, pero la persona que llama no lo
    // sabe
    Tweet {
        username: String::from("Byron"),
        content: String::from("Algun contenido"),
        reply: false,
        retweet: false
    }
}

// Pero esto sólo funciona si se devuelve solo un tipo, el siguiente codigo no funciona
/*
fn return_summarizable2(switch: bool) -> impl Summary {
    if switch {
        Tweet {
            username: String::from("Byron"),
            content: String::from("Algun contenido"),
            reply: false,
            retweet: false
        }
    } else {
        NewsArticle {
            author: String::from("Byron"),
            content: String::from("Contenido"),
            location: String::from("Location"),
            headline: String::from("Algo")
        }
    }
}
 */