// 定义常量必须标明常量类型 如 u32
const MAX_NUM: u32 = 100000;
fn main() {
    println!("MAX_NUM is {}", MAX_NUM);

    // 未使用mut关键字声明变量不能重新赋值编译包错：cannot assign twice to immutable variable
    // let x = 5;
    // x = 6;

    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // 变量隐藏: 此处定义的x会隐藏前面定义的x
    let x = x + 1;
    println!("x = {}", x);

    // 相同变量名可使用let重复定义
    let sapces = "     ";
    println!("sapces is {}", sapces);
    let sapces = sapces.len();
    println!("sapces len is {}", sapces);
}
