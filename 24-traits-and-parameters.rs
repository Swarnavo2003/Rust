trait Summary {
  fn summarise(&self) -> String {
    return String::from("Summarise");
  }
}
struct  User {
  name: String,
  age: u32,
}
struct Fix;
impl Summary for Fix{}
impl Summary for User {
  fn summarise(&self) -> String {
    return format!("The name is {}, and age is {}", self.name, self.age);
  }
}
fn notify(u: &impl Summary) {
  println!("{}", u.summarise());
}
fn main() {
  let user = User {
    name: String::from("Swarnavo"),
    age: 21,
  };
  let f = Fix;
  println!("{}", user.summarise());
  notify(&user);
  notify(&f);
}
