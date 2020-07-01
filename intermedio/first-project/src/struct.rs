// Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// Units struct
// No requieren campo, son útiles para los genéricos.
struct Nil;

// La anotación derive que pueden añadir un comportamiento útil a nuestros tipos personalizados.
// Permiten añadir rasgos y sus comportamientos
#[derive(Debug)] // Es una anotacion
struct Rectangle {
    width: u32,
    height: u32
}

// Implementación
impl Rectangle {
    // Metodo
    // Los métodos te permiten especificar el comportamiento que tienen las instancias de tus estructuras
    // Se definen en el contexto de la estructura (o un objeto enum o un rasgo)
    // y su primer parametro siempre es self, que representa la instancia de la estructura
    // en la que se esta invocando el metodo
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        rect2.width < self.width && rect2.height < self.height
    }

    // Funciones asociadas
    // Se llaman funciones asociadas porque están asociadas a la estructura.
    // Permiten la funcionalidad de espacio de nombres que es particular a la estructura sin tener una instancia
    // disponible
    // Siguen siendo funciones, no métodos, porque no tienen una instancia de la estructura con la que trabajar
    // Estas se utilizan a menudo para constructores que devolverán una nueva instancia de la estructura
    // Para llamar a esta función asociada, usamos la sintaxis :: con el nombre de la estructura
    // Esta función está en el namespace de la estructura: la
    // sintaxis :: se utiliza tanto para las funciones asociadas como para los namespaces creados por los
    // módulos.
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Cada struct puede tener varios bloques impl
impl Rectangle {
    fn perimeter(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }
}

fn main () {
    let user1 = User {
        email: String::from("user1@mail.com"),
        username: String::from("user1"),
        sign_in_count: 1,
        active: true
    };

    // La sintaxis .. especifica que los campos restantes que no se han fijado explícitamente deben
    // tener el mismo valor que los campos de la instancia indicada.
    let user2 = User {
        email: String::from("user2@mail.com"),
        username: String::from("username"),
        ..user1
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    // Destructurando
    let Rectangle{width: my_width, height: my_height} = rect1;
    println!("El rectangulo es ancho = {}, alto = {}", my_width, my_height);
    //
    println!("El area del rectangulo es {}", area(&rect1));
    println!("El rectangulos es {:?}", rect1);
    println!("El rectangulos es {:#?}", rect1);
    // Cuando se llama a un método con object.something(), Rust añade
    // automáticamente &, &mut, o * para que el objeto coincida con la firma del método.
    // Por lo tanto la siguientes linea es lo mismo
    println!("EL area del rectangulo es {}", (&rect1).area());
    println!("EL area del rectangulo es {}", rect1.area());

    // Para llamar a esta función asociada, usamos la sintaxis :: con el nombre de la estructura
    // No hay ninguna razón para separar estos métodos en múltiples bloques impl, pero esta es una
    // sintaxis válida.
    let sq = Rectangle::square(3);
    println!("El perímetro es {}", sq.perimeter());

    // Instanciando una unit struct
    let _nil = Nil;
}

// Construye un Usuario
fn build_user(email: String, username: String) -> User {
    // Debido a que el campo email y el parámetro email tienen el mismo nombre, sólo
    // necesitamos escribir email en lugar de email: email
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}