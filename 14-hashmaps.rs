use std::collections::HashMap;

fn main() {
  let mut users:HashMap<String, u32> = HashMap::new();
  users.insert(String::from("swarnavo"), 21);
  users.insert(String::from("raman"), 31);
  
  let first_user_age = users.get("swarna"); // Option<21>
  match first_user_age {
      Some(age) => println!("age is {}", age),
      None => println!("User not found"),
  }
}
