fn main() {
    let s = String::from("Hello world");
    let world = first_world(&s);
    println!("{}", world);

    let s1 = "Hello world";
    let world = first_world(s1);
    println!("{}", world);

    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[0..3];
    println!("arr_slice is {:?}", arr_slice);
}
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
