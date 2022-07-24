fn main() {
    let s1 = String::from("Hello");
    let len = calcuate_length(&s1);
    // calcuate_length函数借用 s1 并不会使得 s1 丢失所有权在这里仍然可用使用
    println!("the length of '{}' is {}", s1, len);

    let mut s = String::from("Hello");

    let s2 = &mut s;
    // 可变变量不能同时存在多个可变借用
    // let s3 = &mut s; // cannot borrow `s` as mutable more than once at a time second mutable borrow occurs here
    s2.push_str(", World");

    println!("s2 is {}", s2);
    println!("s is {}", s);

    // 借用规则
    // 1、一个可变引用
    // 2、任意数量不可变引用
}

fn calcuate_length(s: &String) -> usize {
    // s.push_str(", World"); 不可修复不可变引用
    s.len()
}
