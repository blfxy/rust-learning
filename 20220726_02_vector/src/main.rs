enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 4];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("{:?}", v);
    println!("{}", v[0]);
    println!("{:?}", v.get(2));

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.34),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
}
