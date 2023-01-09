// 構造体やenumはフルパスで持ち込む
use std::collections::HashMap;

// 同じ名前のモジュールにはasを使用して回避できる
use std::fmt::Result;
use std::io::Result as IoResult;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        // seasonal_fruitは非公開
        seasonal_fruit: String,
    }
    pub enum Appetizer {
        // 列挙子は全て公開される
        Soup,
        Salad,
    }

    impl Breakfast {
        // 非公開のフィールドを持つ構造体
        // を作成する関連関数
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// 別ファイルから読み込むようにする
mod front_of_house;
// 関数は親モジュールまで持ち込む
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // useによる持ち込み
    hosting::add_to_waitlist();

    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 下の行のコメントを外すとコンパイルできない。食事についてくる
    // 季節のフルーツを知ることも修正することも許されていない
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
