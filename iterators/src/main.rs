fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // ここで初めて繰り返しが起きる
    // v1_iterの所有権は奪われる
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();
    println!("{}", total);

    let v3: Vec<i32> = vec![1, 2, 3];
    let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();
    dbg!(v4);
}
