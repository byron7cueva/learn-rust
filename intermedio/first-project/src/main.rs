// Tipos de datos compuestos
// Son grupos de valores que pueden tener tipos totalmente diferentes
// Una vez definidos no se puden extender, es decir si se lo crea con 3 valores no se puede añadir

fn main () {
  // Tuplas
  // Definiendo una tupla
  let t = (1, "", 2.3);

  // Accediendo al valor de la posicion 0
  let x = t.0;
  println!("{}", x);

  let (num, cadena, flotante) = t;
  println!("{}, {}, {}", num, cadena, flotante);

  let s = "hola mundo".to_string();
  let (a, b) = s.split_at(5);
  println!("{}, {}", a, b);

  // Tupla cero o unit
  // Esta es la que se devuelve por defecto en funciones que no devuelven un valor
  let tupla = ();
  let r = f("1asas");
  println!("resulado {:?}", r);

  // Array
  // Deben ser los elementos del mismo tipo
  // Tienen un tamaño fijo no se pueden extender ni encojer, si se quiere hacer se deve utilizar un vector
  // Para acceder se debe utilizar el indice
  // El indice tiene que ser siempre de tipo usize
  // Estan asignados en el stack y no en el heap
  let array = [1, 2, 3];
  // Declarando un array de tipo i32 de 6 elementos
  let array2: [i32; 6] = [1, 2, 3, 4, 5, 6];
  // Creando un array de 100 elementos todos ellos con el valor 0
  let array3 = [0u8; 100];
  println!("{}", array[0]);
  let i = 1;
  println!("{}", array[i]);
  let indice: usize = 10;
  // get nos permite acceder de manera segura
  // Devuelve un option que puede ser un valor Some(valor_elemento) o si no existe un elemento en el indice
  // Devuelve un None
  println!("{:?}", array.get(indice));

  // Nos permite acceder a un indice asi no exista
  // Retorna unsafe Insegura: Es para uso avanzado
  // Deshabilitando la seguridad de rust
  unsafe {
    println!("{:?}", array.get_unchecked(indice));
  }
}

fn f(s: &str) -> Result<(), Box<dyn std::error::Error>> {
  let n = s.parse::<i32>()?;
  print!("{}", n);
  Ok(())
}