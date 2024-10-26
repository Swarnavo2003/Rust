use std::collections::HashMap;

fn group_values_by_keys(vec:Vec<(String,i32)>) -> HashMap<String, i32> {
  let mut hm = HashMap::new();
  for (key, value) in vec {
    hm.insert(key, value);
  }
  return hm;
}

fn main() {
  let input_vec = vec![(String::from("swarnavo"),21),(String::from("raman"),32)];
  println!("{:?}", input_vec);

  let hm = group_values_by_keys(input_vec);
  println!("{:?}",hm)
}
