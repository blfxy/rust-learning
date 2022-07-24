fn main() {
    // 元组
    let tuple_e = ('e', 5i32, true);
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );
    // 元组解构
    let (x, y, z) = tuple_e;
    println!("x={}, y={}, z={}", x, y, z);

    // 数组
    let arr = [3; 5];
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    println!("{:?}", arr1);
    // 矢量类型
    let vec = vec![1, 3, 5];
    println!("{:?} {}", vec, vec[0]);
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    // vec1.push("2");
    println!("{:?},{:?}", vec1, vec1[1]);
    vec1[0] = vec1[0] + vec1[1];
    vec1.pop();
    println!("{:?}", vec1);

    
}
