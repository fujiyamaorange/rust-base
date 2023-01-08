fn main() {
    let s1 = String::from("hello");

    // borrowing
    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);

    // let s2 = String::from("hello");

    // try_change(&s2);

    let mut s3 = String::from("hello");

    will_change(&mut s3);
    // 特定のスコープで可変な参照は1つしか作成できない(不変な参照はOK)
    let _r1 = &mut s3;
    // let r2 = $mut s3; // これはできない
}

fn calculate_length(s: &String) -> usize {
    // 参照しているものの所有権を持っているわけではない
    s.len()
}

fn try_change(_some_string: &String) {
    // 借用したものは変更できない
    // some_string.push_str(", world");
}

fn will_change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ダングリング
fn dangle() -> &String {
    // dangleはStringへの参照を返す

    let s = String::from("hello"); // sは新しいString

    &s // String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消されるので危険
