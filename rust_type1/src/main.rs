fn main() {
    // 整数数字 8 16 32 64 128 i有符号/u无符号 isize usize 与体系结构相关
    // 浮点数 f32 f64
    // 布尔值
    // 字符
    let number: u32 = 14;
    // let number: &str = "14";
    println!("the number is {}", number);
    let number_64 = 4.0;
    let number_32 = 5.0;
    println!("{}-{}", number_32, number_64);
    println!(
        "1+2 = {} and 8-5 = {} and 15*3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
    let is_bigger = 1 > 4;
    println!("is 1 > 4 ? {}", is_bigger);
    let character_1: char = 's';
    let character_2: char = 'f';
    let simley_face = '😀';
    let string_1 = "miley ";
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}",
        simley_face, character_1, string_1, character_2, string_2
    );
}
