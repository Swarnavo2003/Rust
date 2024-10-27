fn main() {
  let v1 = vec![1,2,3];
  let v1_iter = v1.iter();

  let sum: i32 = v1_iter.sum(); // Its takes ownership and consumes the variable v1_iter we cannot use it anymore in the code
  println!("sum is {}", sum);
  println!("{:?}",v1);
}
