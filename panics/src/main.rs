use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // 攻撃者が、配列の後に格納された読めるべきでないデータを読み出せるように添え字を操作できたら、
    // セキュリティ脆弱性につながる可能性があります
    // let v = vec![1, 2, 3];
    // v[99];

    // Okなら中身を，Errならpanicを起こす
    let f = File::open("hello.txt").unwrap();
    // Err時のメッセージを設定できる
    let f = File::open("hello.txt").expect("This is custom error message.");

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                }
            }
        }
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err);
        }
    };
}

// Resultを返す
// エラーを委譲する
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

    // ?を使って連結して書ける
    // Resultを返す関数内で使用できる
    File::open("hello.txt")?.read_to_string(&mut s)?;
}
