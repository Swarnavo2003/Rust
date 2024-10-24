struct User {
    first_name: String,
    last_name: String,
    age: i32,
}
fn main() {
    let user = User {
        first_name: String::from("Swarnavo"),
        last_name: String::from("Majumder"),
        age: 21,
    };
    println!("{}",user.first_name);
    println!("{}",user.last_name);
    println!("{}",user.age);
}
