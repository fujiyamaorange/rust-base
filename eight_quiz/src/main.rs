use std::collections::HashMap;

// https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html#%E3%81%BE%E3%81%A8%E3%82%81
fn main() {
    // QUESTION1
    let mut scores = vec![8, 12, 13, 5, 19, 15, 17, 10, 8, 11, 16, 15, 13, 1, 15, 15];
    let length = scores.len() as f32;
    let mut sum: f32 = 0.0 as f32;
    let mut map = HashMap::new();
    let mut mode = -1;
    let mut mode_count = 0;

    for score in &scores {
        // sum
        sum += *score as f32;

        // mode
        let count = map.entry(score).or_insert(0);
        *count += 1;
    }

    for (&key, &value) in map.iter() {
        // println!("{}: {}", key, value);
        if value > mode_count {
            mode = *key;
            mode_count = value;
        }
    }

    // println!("length: {}", length);
    // println!("sum: {}", sum);
    println!("mean: {:.4}", sum / length);

    // println!("{:#?}", map);
    println!("mode: {}", mode);

    // median
    scores.sort();
    println!("{:?}", scores);
    println!("median: {}", &scores[(length / 2.0) as usize]);

    // QUESTION2
    // let word = "apple";
    let word = "first";
    let result = pig_latin(word);
    println!("{}", result);

    // QUESTION3
}

fn pig_latin(word: &str) -> String {
    let vowel = vec!["a", "i", "u", "e", "o"];
    let initial = &word[..1];

    if vowel.iter().any(|v| v == &initial) {
        // 母音あり
        let mut name = String::from(word);
        name.push_str("-hay");
        return name;
    } else {
        // 母音なし
        let mut name = String::from(&word[1..]);
        name.push_str("-");
        name.push_str(initial);
        name.push_str("ay");
        return name;
    }
}
