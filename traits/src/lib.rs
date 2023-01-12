pub trait Summary {
    // メソッドシグニチャ
    // fn summarize(&self) -> String;

    // デフォルトの実装を書くこともできる
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 型にトレイトを実装する
impl Summary for NewsArticle {
    // デフォルトのsummarizeを使う
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// localの型のみ実装できる
// Vec<T>にDisplayトレイトは実装できない
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// トレイト(ここではSummary)に由来するインスタンスならなんでも受け付ける
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 上は下の糖衣構文
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 以下のようの複数のトレイトを+で指定できる
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}

// 戻り値としてトレイトを実装した型を指定できる
// 1種類の型しか返せない(条件によってNewArticleかTweetかなどはできない)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
