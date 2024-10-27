fn filter_and_map(v:Vec<i32>) -> Vec<i32> {
  let new_iter = v.iter().filter(|x| *x%2==1).map(|x| x*2);
  let new_vec:Vec<i32> = new_iter.collect();
  return new_vec;
}

fn main() {
  let v1 = vec![1,2,3,4,5,6,7,8,9,10];
  let ans = filter_and_map(v1);
  println!("{:?}",ans);
}
