fn main () {
  // Del 1 al 10
  for i in 1..11 { // Not inclusive
    println!("The number is {}", i);
  }

  let numbers = 30..51; // This is array
  for i in numbers {
    println!("The number is {}", i);
  }

  let animals = vec!["Rabit","Dog","Cat"]; // Array de strings 
  for a in animals.iter() { // Recorriendo un iterable
    println!("The animal is {}", a);
  }

  for (index, a) in animals.iter().enumerate() {
    println!("The index is {} and the animal name is {}", index, a);
  }
}