fn main() {
    println!("Hello, world!");

    let num = plus_hundred(make_five());
    another_function(num);
}

fn make_five() -> i32 {
    5
}

fn plus_hundred(x: i32) -> i32 {
    x + 100
}

// どこで定義しても良い
fn another_function(x: i32) {
    // 式は終端にセミコロンを含みません
    println!("The parameter is {}", x);
}
