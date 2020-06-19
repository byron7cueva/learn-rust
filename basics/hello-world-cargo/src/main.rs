fn main() {
  /* Replace */
  {
    let my_string = String::from("Rust is fantastic");
    println!("After replace {}", my_string.replace("fantastic", "great"));
  }

  // Lines
  {
    let my_string = String::from("The weather is\nnice\noutsite mate!");

    for line in my_string.lines() {
      println!("[ {} ]", line);
    }
  }

  // Split
  {
    let my_string = String::from("Leave+a+like+if+you+enjoyed");
    let tokens: Vec<&str> = my_string.split("+").collect();

    println!("{}", my_string);
    println!("At index 2: {}", tokens[2]);
  }

  // Trim
  {
    let my_string = String::from("    My name is Byron   \n\r  ");
    println!("Before trim: {}", my_string);
    println!("After trim {}", my_string.trim());
  }

  // Chars
  {
    let my_string = String::from("Im developer");
    println!("{}", my_string);

    match my_string.chars().nth(4) {
      Some(c) => println!("Character at index 4: {}", c),
      None => println!("Don't has character at index 4")
    }
  }
}