// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
//     println!("{}", s);

//     let s2 = s;
//     println!("{}", s); // 使えない(二重開放を避ける)

//     let x = 5;
//     // スタックに存在するデータなので使える
//     let y = x;

//     println!("x = {}, y = {}", x, y);
// }

// fn main() {
//     let s = String::from("hello");

//     // sの値が関数にムーブ
//     takes_ownership(s);
//     // ここではもう有効ではない→コンパイルエラー
//     // println!("{}", s);

//     let x = 5; // xがスコープに入る

//     makes_copy(x);
//     // スタックなので使用できる
//     println!("{}", x);
// } // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn main() {
    let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1に
                                // ムーブする

    let s2 = String::from("hello"); // s2がスコープに入る

    // s2はtakes_and_gives_backにムーブされ戻り値もs3にムーブされる
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2);
    println!("{}", s3);
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {
    // gives_ownershipは、戻り値を
    // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    // some_stringが返され、呼び出し元関数にムーブされる
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_stringがスコープに入る。
    a_string // a_stringが返され、呼び出し元関数にムーブされる
}
