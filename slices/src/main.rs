// fn first_word(s: &String) -> usize {
//     // バイト配列に変更
//     let bytes = s.as_bytes();

//     // enumerateで添字と要素への参照を取得する
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// strはスライスを表す型
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("Hello World!");
    let word = first_word_slice(&s[..]);

    // 関数のシグニチャを変更することで文字列リテラルも扱える
    let s2 = "Next World!";
    let word2 = first_word_slice(&s2[..]);

    // 不変な参照がある場合，可変な参照を使えない
    // s.clear();

    println!("{}", word);
    println!("{}", word2);
}
