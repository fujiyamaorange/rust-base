use std::collections::HashMap;

fn main() {
    // ========================================================
    // let v: Vec<i32> = Vec::new();
    // vecマクロ
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 値取得方法1
    let fifth: &i32 = &v[4];
    println!("The fifth element is {}", fifth);

    // 値取得方法2
    // .getはOption<&T>を返す
    let second = v.get(1);
    match second {
        Some(value) => println!("The value is {}", value),
        None => println!("There is no value."),
    }

    // panic!!
    // let does_not_exist = &v[100];
    // println!("The hundred element is {}", does_not_exist);

    // Noneを返す
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(value) => println!("The value is {}", value),
        None => println!("The value is not exists."),
    }

    let mut vec = vec![1, 2, 3, 4, 5];

    // 不変参照
    let first = &vec[0];

    // 可変借用はできない
    /**
     * なぜ最初の要素への参照が、ベクタの終端への変更を気にかける必要があるのでしょうか？
     * このエラーはベクタが動作するしくみによるものです。
     * 新たな要素をベクタの終端に追加するとき、
     * いまベクタのある場所に全要素を隣り合わせに配置するだけのスペースがないなら、
     * 新しいメモリを割り当て、古い要素を新しいスペースにコピーする必要があります。
     * その場合、最初の要素を指す参照は、解放されたメモリを指すことになるでしょう。
     * 借用規則がそのような状況に陥らないよう防いでくれるのです。
     */
    // vec.push(6);

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // iの値に辿り着く必要がある
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
        println!("{}", *i);
    }
    for i in v {
        println!("{}", i);
        // println!("{}", *i);
    }

    // enumを使って異なる型の値をvecに保持する
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // ========================================================

    // %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
    let mut s = String::new();

    let data = "initial contents";
    // 文字列リテラルからStringを生成する
    let mut s = data.to_string();
    // 引数の所有権はなくて良い
    s.push_str("here");

    // UTF-8でエンコードできるものであればなんでもOK
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // &Stringを&strに型矯正(String + &strが成り立つ)→参照外し型矯正
    let s3 = s1 + &s2;
    // s1はムーブされ、もう使用できないことに注意
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 引数の所有権を奪わない
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // let hello = "Здравствуйте";
    let hello = "Hola";
    // 添字アクセスはできない
    // スライスでアクセス(panicの可能性もある)
    let answer = &hello[0..3];
    println!("{}", answer);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // collectを使っても初期化できる
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    let team_name = String::from("Blue");
    // getで取り出す→Option<&V>
    let score = scores.get(&team_name);
    println!("{:?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Stringのように所有権のある値なら所有権はハッシュマップに移動する
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // できない
    // println!("{}", field_name);

    let mut scores = HashMap::new();

    // 一度しか追加されない
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // entry&or_insert→キーがなければ追加する
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    let mut dict: HashMap<_, String> = HashMap::new();
    for word in text.split_whitespace() {
        // or_insertは可変参照(&mut V)を返す
        let count = map.entry(word).or_insert(0);
        // 借用規則に則って更新
        *count += 1;
    }
    for word in text.split_whitespace() {
        // 先頭の文字を追加
        let c = dict
            .entry(word)
            .or_insert(String::from(word.chars().next().unwrap()));
        // 借用規則に則って更新
        // (*c).push_str("+");
        c.push_str("+");
    }
    println!("{:?}", map);
    println!("{:?}", dict);

    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
