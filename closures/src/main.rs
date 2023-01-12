use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // closure(型注釈の必要はない)
    // クロージャを保持するCacherのインスタンスを作成
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity + 1));
        println!("Next, do {} situps!", expensive_result.value(intensity + 2));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// クロージャはFnトレイトを実装する
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    // value: Option<u32>,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                // HashMapに入れる
                self.value.insert(arg, arg);
                arg
            }
        }
    }
}

// クロージャと関数の違い
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    // クロージャ内で同スコープの変数を使用できる
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
