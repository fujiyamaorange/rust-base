// ライブラリクレートをバイナリクレートのスコープに持っていく
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // collectは型注釈が必要になることが多い
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 標準エラー出力(stderr)
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // Okの中身は使用しないのでif letの形を使う
    if let Err(e) = minigrep::run(config) {
        // 標準エラー出力(stderr)
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
