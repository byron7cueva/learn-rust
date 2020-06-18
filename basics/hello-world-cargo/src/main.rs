// Vector

fn main () {
  // Create a vector
  let vector: Vec<i32> = Vec::new();
  let mut vector2 = vec![1, 2, 3, 4];

  println!("{}", vector2[3]);

  vector2.push(49);
  // Remove elemento form vector, pass the index
  vector2.remove(1); // Remove 2

  for number in vector2.iter() {
    println!("{}", number);
  }
}