fn main() {
  let v1 = vec![1,2,3];
  let v1_iter = v1.iter();

  // ---------- map ----------
  // let v1_iter2 = v1_iter.map(|x| x + 1); // this returns an iterator
  // for i in v1_iter2 {
  //   println!("{}",i);
  // }

  // ----------- filter ----------
  let v1_iter2 = v1_iter.filter(|x| *x % 2 == 0); // this returns an iterator
  for i in v1_iter2 {
    println!("{}",i);
  }

  println!("{:?}",v1);
}
