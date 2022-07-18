fn main() {
    // 数组
    let arr = [0; 5];
    let arr1 = [6, 5];
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
