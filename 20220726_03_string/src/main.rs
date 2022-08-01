use std::iter::from_fn;

fn main() {
    let mut s = String::new();
    let s = "Hello World".to_string();
    let mut s = String::from("Hello");
    s.push_str(", World");
    s.push('s');
    println!("{}", s);
    let s1 = String::from("Hello,");
    let s2 = String::from("World");
    // let s3 = s1 + &s2;
    // println!("{}", s3);
    let s = format!("{}-{}", s1, s2);
    println!("{}", s);
}
