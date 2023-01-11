use std::fmt::Display;

// &i32        // (ただの)参照
// &'a i32     // 明示的なライフタイム付きの参照
// &'a mut i32 // 明示的なライフタイム付きの可変参照

// 引数の全ての参照と戻り値が全て同じライフタイムを持つことを示す
// 元の値のライフタイムは変更していない
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 関数から参照を返す際、戻り値型のライフタイム引数は、引数のうちどれかのライフタイム引数と一致する必要があります。
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// <'a>注釈はImportantExcerptのインスタンスがpartフィールドに保持している参照よりも長生きしない
struct ImportantExcerpt<'a> {
    // フィールドに参照を保持する(ライフタイム付き)
    part: &'a str,
}

// ジェネリクス＆トレイト境界＆ライフタイム
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    // string1とstring2はライフタイムが違うがコンパイルできる
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let string3 = String::from("long string is long");
    let result;
    {
        let string4 = String::from("xyz");
        result = longest(string3.as_str(), string4.as_str());
    }
    // resultは小さい方(string4)と同じライフタイムになるためコンパイルできない
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Rustの改善により一部ではライフタイムを省略することができるようになった
    // 以下の3つの規則に当てはまる場合はコンパイラがライフタイムを計算
    // ====
    // 最初の規則は、参照である各引数は、独自のライフタイム引数を得るというものです。換言すれば、 1引数の関数は、1つのライフタイム引数を得るということです: fn foo<'a>(x: &'a i32); 2つ引数のある関数は、2つの個別のライフタイム引数を得ます: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 以下同様。
    // 2番目の規則は、1つだけ入力ライフタイム引数があるなら、そのライフタイムが全ての出力ライフタイム引数に代入されるというものです: fn foo<'a>(x: &'a i32) -> &'a i32。
    // 3番目の規則は、複数の入力ライフタイム引数があるけれども、メソッドなのでそのうちの一つが&selfや&mut selfだったら、 selfのライフタイムが全出力ライフタイム引数に代入されるというものです。 この3番目の規則により、必要なシンボルの数が減るので、メソッドが遥かに読み書きしやすくなります
    // ====

    // 静的ライフタイム
    let s: &'static str = "I have a static lifetime.";
}
