fn main() {
    let num = 3;
    // if后面的表达式必须是布尔值
    if num < 5 {
        println!("conditon was true");
    } else {
        println!("conditon was false")
    }

    let num = 6;
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let num = 7;
    match num {
        _ if num % 4 == 0 => {
            println!("num 可以被4整除");
        }
        _ if num % 3 == 0 => {
            println!("num 可以被3整除");
        }
        _ if num % 2 == 0 => {
            println!("num 可以被2整除");
        }
        _ => {
            println!("num 不能被 4, 3，2整除");
        }
    }

    // if 是个表达式;
    let conditon = true;
    let num = if conditon { 5 } else { 6 };
    println!("the value if num is: {}", num);
}
