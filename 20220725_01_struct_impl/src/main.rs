#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 以self作为第一个参数的是方法, 不已self为第一个参数的是关联函数
impl Rectangle {
    // 方法1
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 方法2
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 60,
    };
    let rect1 = Rectangle {
        width: 50,
        height: 500,
    };
    println!("{}", rect.area());
    println!("{}", rect.can_hold(&rect1));
    println!("{}", rect1.can_hold(&rect));
    let s = Rectangle::square(20);
    println!("{}", s.area());
    println!("{:?}", s);
}
