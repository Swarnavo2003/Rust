fn main() {
  // Ownership: Can be only one owner
  let s1 = String::from("swarnavo");

  do_something(s1);
  // println!("number is {}",s1); // will give a error
}

fn do_something(s2:String) {
  println!("{}", s2);
}
