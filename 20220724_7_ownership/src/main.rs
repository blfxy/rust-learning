fn main() {
    // 每个值都有一个变量，这个变量是该值的所有者
    // 每个同时只能有一个所有者
    // 当所有者超出作用域时，该值将被删除

    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{}", s);

    // 整数是已知固定大小的简单值，这两个5被压到stack中
    let x = 5;
    let y = x;

    // String类型是复杂数据结构，内容存放在堆中，栈中存放指向堆中内容的指针, 包括：ptr(堆内存的地址) len(堆中内容长度) capacity(操作系统分配的内存大小)
    // s1赋值给s2只复制栈而不会复制一份堆 (所有权移动到了s2，s1失效，不再可用)
    let s1 = String::from("Hello");
    let s2 = s1;
    // s1 移动到了s2 s1不再可以使用
    // println!("{}", s1); // error: borrow of moved value: `s1` value borrowed here after move

    // 使用clone方法会复制包括堆内存中的数据 不会导致 s2 失效
    let s3 = s2.clone();
    println!("s2 is {}, s3 is {}", s2, s3);

    let s = String::from("Hello, World");
    take_ownership(s); // s移动到 take_ownership 内，
                       // s 在这里开始不再可用使用

    let x = 5;
    makes_copy(x);
    println!("x: {}", x);
}

fn hello() {
    // s不可用
    let _s = "hello"; // _s 可用
                      // 可以对_s进行相关操作
} // _s作用域到这里结束，_s不再可用 rust 会调用 drop() 自动将内存返还给操作系统

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}
