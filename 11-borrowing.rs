fn main() {
  let s1 = String::from("swarnavo");
  do_something(&s1)
}

fn do_something(s2:&String) {
  println!("{}", s2);
}
