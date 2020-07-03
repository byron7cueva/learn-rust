// Generics
// Structure
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U
}

// Enums
enum SomeEnum<T> {
    OptionA(T),
    OptionB(T),
    OptionC
}

// trait
trait SomeCustomTrait {
    fn blah_blash(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct DougsStruct {
    something: i32
}

impl SomeCustomTrait for DougsStruct {
    fn blah_blash(&self, a: &str, b: &str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}

impl SomeCustomTrait for i32 {
    fn blah_blash(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

//Impl
// declarar T justo después de impl para poder utilizarlo y especificar
// que estamos implementando métodos en el tipo Point<T>
impl<T> Point<T> {
    // hemos definido un método llamado x en Point<T> que devuelve una referencia a los datos en
    // el campo x
    fn x(&self) -> &T {
        &self.x
    }
}

// Podríamos implementar métodos sólo en instancias de Point<f32> en lugar de en
// instancias de Point<T> con cualquier tipo genérico
// Este código significa que el tipo Point<f32> tendrá un método llamado distance_from_origin y
// otras instancias de Point<T> donde T no es de tipo f32 no tendrán este método definido
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Los parámetros genéricos de tipo en una definición de estructura no siempre son los mismos que los
// que utiliza en las firmas de método de esa estructura
struct Point4<T, U> {
    x: T,
    y: U
}

// Los parámetros genéricos T y U se declaran después de impl, porque van con la definición de la estructura
// Los parámetros genéricos V y W se declaran después de fn mixup, porque sólo son relevantes para el
// método
impl<T, U> Point4<T, U> {
    fn mixup<V, W>(self, other: Point4<V, W>) -> Point4<T, W> {
        Point4 {
            x: self.x,
            y: other.y
        }
    }
}

struct DougsStruct2<T, U>
    where T: std::fmt::Debug,
          U: std::fmt::Debug {
    dougs_t: T,
    dougs_u: U
}

impl<T, U> DougsStruct2<T, U>
    where T: std::fmt::Debug,
          U: std::fmt::Debug {
    fn log_somethings(&self) {
        println!("{:?} {:?}", self.dougs_t, self.dougs_u);
    }
}

fn main () {
    // Creando objeto implicitamente con sus atributos tipo i32
    // let a = Point { x: 100, y: -1}
    // Creaando objeto explicitamente con sus atributos de tipo i32
    let a: Point<i32> = Point { x: 100, y: -1};
    println!("x = {}, y = {}", a.x, a.y);

    let b = Point { x: 10.1, y: -2.3};
    print!("x {}, y {}", b.x, b.y);

    let c = Point2 { x: 2, y: 3.5 };
    println!("x {}, y {}", c.x, c.y);

    // Creando con la OpcionA definiendola de f64
    let some_data = SomeEnum::OptionA(34.2);

    match some_data {
        SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
        SomeEnum::OptionB(b) => println!("OptionB contains {}", b),
        SomeEnum::OptionC => println!("OptionC don't contains")
    }

    let some_data2 = SomeEnum::OptionB('b');
    let some_data3 = SomeEnum::OptionA(vec![1, 2, 3]);

    // let d = dougs_fun(4, 5);
    // Al indicar que es i8 produce una funcion con ese tipo de dato
    let d = dougs_fun(4 as i8, 5);
    println!("d has {}", d);

    let test = DougsStruct { something: 1000 };
    let result = do_this(&test);

    let testi32 = 18;
    let result2 = do_this(&testi32);

    let test_dougs = DougsStruct2 {
        dougs_t: 5.6,
        dougs_u: vec![1, 2, 3]
    };

    test_dougs.log_somethings();

    let some_point = Point {x: 20, y: 10};
    println!("point.x = {}", some_point.x());

    let p1 = Point4 { x:5, y: 10.4};
    let p2 = Point4 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Las funciones se deben definir debajo del main

// Function
/* fn dougs_fun<T>(input_a: T, input_b: T) -> T {
  input_a + input_b // Esto no se puede hacer ya que entre genericos no se puede sumar
} */

//Constraint: <T: std::ops::Add<Output=T>> Indicando que se puede sumar y el tipo del resultado va ser T
/* fn dougs_fun<T: std::ops::Add<Output=T>>(input_a: T, input_b: T) -> T {
  input_a + input_b
} */

// Agregando constraint para que ademas se pueda restar
/* fn dougs_fun<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T>>(input_a: T, input_b: T) -> T {
  input_a - input_b
} */

fn dougs_fun<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug>(input_a: T, input_b: T) -> T {
    println!("input_a has {:?}", input_a);
    input_a - input_b
}

#[allow(dead_code)] // Para que no me de alvertencia de que no se esta utilizando la funcion
fn dougs_fun2<T>(input_a: T, input_b: T) -> T
    where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug {
    println!("input_a has {:?}", input_a);
    input_a + input_b
}

#[allow(dead_code)]
fn dougs_fun3<T, E>(input_a: T, input_b: T, input_e: E) -> T
    where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug,
          E: std::fmt::Debug {
    println!("input_a has {:?}", input_a);
    println!("input_e has {:?}", input_e);
    input_a + input_b
}

fn do_this<T>(some_var: &T) -> String
    where T: SomeCustomTrait + std::fmt::Debug {
    println!("some_var {:?}", some_var);
    some_var.blah_blash("first", "second")
}

/* fn do_this2(some_var: &dyn SomeCustomTrait) -> String {
  // Some complex logic
  some_var.blah_blash("first", "second")
} */