fn main() {
    // println!("Hello, world!");
    // Display the message "Hello, world"
    // todo!("Display the message by using the println!() macro");
    // println!(
    //     "The first letter of the English alphabet is {} and the last letter is {}",
    //     'a', 'z'
    // );
    // mut 关键字 表示定义的变量 可修改
    let mut a_number;
    let a_word = "Ten";
    a_number = 10;
    println!("The number is {}", a_number);
    println!("The word is {}", a_word);
    a_number = 15;
    println!("The number is {}", a_number);

    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;
    println!("the number is {}", shadow_num);
}
