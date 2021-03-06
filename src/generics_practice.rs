use std::fmt::Display;

/// 泛型練習
pub(crate) fn run() -> () {
    let interger = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("p.x = {}", interger.x());
    println!("p distance from origin = {}", float.distance_from_origin());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!(
        "1 則新推文：{} 類別：{}",
        tweet.summarize(),
        tweet.category()
    );

    let news = NewsArticle {
        headline: String::from("標題"),
        location: String::from("地點"),
        author: String::from("作者"),
        content: String::from("內文"),
    };

    println!("1 則新聞：{} 類別：{}", news.summarize(), news.category());
    notify(&tweet);
    notify_and_display(&news);
    notify_and_display2(&news);

    let some_summarizable = create_summarizable();
    println!("some_summarizable: {}", some_summarizable.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("最大數字為 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("最大字元為 {}", result);

    let pair = Pair { a: 1, b: 2 };
    pair.compare_display();

    //i32 是一種 Display, Display 被實作了 Notifyable
    1.notify_from_notifyable();

    //生命週期問題

    let string1 = String::from("很長的長字串");
    let result;
    let string2 = String::from("xyz");
    //string1 string2 的生命週期不同，而 longest 的設計會導致輸出的 result 的生命週期不定。
    result = longest(string1.as_str(), string2.as_str());
    println!("最長的字串為 {}", result);

    let novel = String::from("叫我以實瑪利。多年以前...");
    let first_sentence = novel.split('.').next().expect("找不到'.'");
    //i 的 part 來自 first_sentence，由於生命週期約束，i的生命週期必須不長於 first_sentence 的生命週期 novel
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

/// 查詢列表中最大的東西
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

///泛型結構範例
struct Point<T> {
    x: T,
    y: T,
}

///為泛型結構實踐方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

///為泛型結構的特定型別實踐方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

///定義特徵 特徵有點像是 interface
pub trait Summary {
    fn summarize(&self) -> String;
    ///有預設實作的方法
    fn category(&self) -> String {
        String::from("一般")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

///為特定型別NewsArticle實踐特徵
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} {} 著 ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

///為特定型別Tweet實踐特徵
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }

    fn category(&self) -> String {
        if self.reply {
            String::from("回文")
        } else {
            String::from("原文")
        }
    }
}

///接受型別有實作特徵的參數的方法
pub fn notify(item: &impl Summary) {
    println!("頭條新聞! {}", item.summarize())
}

///參數型別可以是複合特徵
pub fn notify_and_display(item: &(impl Summary + Display)) {
    println!("summary: {}", item.summarize());
    println!("display: {}", item);
}

/// 使用 where 表示特徵界限
pub fn notify_and_display2<T>(item: &T)
where
    T: Summary + Display,
{
    println!("summary: {}", item.summarize());
    println!("display: {}", item);
}

/// 方法可以回傳特徵
fn create_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    a: T,
    b: T,
}

/// 可以針對複合特徵實踐方法
impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    fn compare_display(&self) {
        if self.a > self.b {
            println!("a:{} > b:{}", self.a, self.b)
        } else if self.a < self.b {
            println!("a:{} < b:{}", self.a, self.b)
        } else {
            println!("a:{} = b:{}", self.a, self.b)
        }
    }
}

trait Notifyable {
    fn notify_from_notifyable(&self) -> ();
}

/// 對滿足特徵界限的型別實作特徵（毯子實作）類似 extension method
impl<T: Display> Notifyable for T {
    fn notify_from_notifyable(&self) -> () {
        println!("Notice! Value is {}", self)
    }
}

/// 當函式的參數與回傳值存在複數引用時，可能會需要標註回傳值的生命週期來自誰
/// rust 的編譯器持續改進後，這種需要提示生命週期的狀況可能會減少
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

///當結構中的欄位存在引用時，需要標記其生命週期，此詮釋代表 ImportantExcerpt 的實例不能比它持有的欄位 part 活得還久。
struct ImportantExcerpt<'a> {
    part: &'a str,
}
