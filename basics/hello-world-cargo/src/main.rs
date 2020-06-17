// Tuples

fn main () {
  // Define a tuple
  let tup1 = (10, 20, 30, 40);

  // Access to value of position
  // Access to 3th position. Initialice from 0
  println!("{}", tup1.2);

  let tup2 = (20, "Rust", 3.4, false);
  println!("{}", tup2.3);

  //A tuple into other tuple
  let tup3 = (20, "Rust", 3.4, false, (1, 4, 7));
  println!("{}", (tup3.4).1);

  //
  let tup4 = (45, 6.7, "Computer", 56);
  let (a, b, c, d) = tup4;

  println!("a is {}", a);
  println!("b is {}", b);
  println!("c is {}", c);
  println!("d is {}", d);
}