fn main() {
    // loop循环
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("the result is: {}", result);

    // while循环

    let mut num = 3;
    while num != 0 {
        println!("{}!", num);
        num = num - 1;
    }
    println!("发射");

    let arr = [10, 20, 30, 40, 50];
    let mut id = 0;
    //索引容易出错
    while id < 5 {
        println!("this value is: {}", arr[id]);
        id = id + 1;
    }

    for v in arr.iter() {
        println!("the value is: {}", v);
    }

    for i in 1..4 {
        println!("{}", i);
    }
    for i in (1..4).rev() {
        println!("{}", i);
    }
    println!("发射");
}
