fn main() {
  let mut v1 = vec![1,2,3];

  // ---------- 1st Way(Deafault into_iter) ----------
  // for val in v1 {
  //   println!("{}", val);
  // }

  // ---------- 2nd Way ---------- 
  // let v1_iter = v1.iter();
  // for val in v1_iter {
  //   println!("{}", val);
  // }

  // ---------- 3rd Way ----------
  // let v1_iter = v1.iter_mut();
  // for val in v1_iter {
  //   *val = *val + 1;
  // }
  
  // ---------- 4th Way ----------
  // let mut v1_iter = v1.iter_mut();
  // while let Some(val) = v1_iter.next() {
  //   println!("{}", val);
  // }

  // ---------- 5th Way ----------
  let v1_iter = v1.into_iter();
  for val in v1_iter {
    println!("{}", val);
  }

  // println!("{:?}",v1); // Will not work in case of 1st and 5th way because the iterators takes the ownership of the vector
}
