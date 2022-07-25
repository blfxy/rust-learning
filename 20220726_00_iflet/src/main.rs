fn main() {
    let v = Some(0u8);
    // 必须匹配所有可能
    match v {
        Some(3) => println!("three"),
        _ => println!("false"),
    }
    let v = Some(3);
    // 只关心一种匹配 忽略其它匹配情况 可以配合else使用
    if let Some(3) = v {
        println!("thress");
    }
}
