fn main() {
  let mut s1 = String::from("swarnavo");
  do_something(&mut s1);
  println!("name is {}", s1);
}

fn do_something(s2:&mut String) {
  s2.push_str(" majumder");
  println!("{}", s2);
}
