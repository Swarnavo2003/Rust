use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();
  scores.insert("Alice", 50);
  scores.insert("Bob", 40);
  scores.insert("Charlie", 30);

  for (key, value) in scores.iter() {
    println!("{}: {}", key, value);
  }

  for (key, value) in scores.iter_mut() {
    *value += 10;
    println!("{}: {}", key, value);
  }
}
