fn main() {
    let formal = "Formal: Good bye";
    let casual = "Casual: see you later!";
    goodbye(formal);
    goodbye(casual);
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}
fn goodbye(message: &str) {
    println!("\n{}", message);
}
fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    num / 5
}
