// Trait는 다른 언어로 비유하자면, 인터페이스와 비슷한 개념이다.


pub trait Summary {
    fn summarize(&self) -> String;
}


/*
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Summary 트레이트를 구현하는 NewsArticle 구조체
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Summary 트레이트를 구현하는 Tweet 구조체
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
*/


/*
pub trait Summary {
    fn summarize(&self) -> String {
        // 기본 구현으로, 이 메소드를 호출하면 아래 문자열을 반환함
        String::from("(Read more...)")
    }
}

// 기본구현 사용시에는, 아래와 같이 impl Summary for NewsArticle {}로만 구현해도 됨
impl Summary for NewsArticle {}

pub trait Summary {
    // 부분적으로 기본 구현을 사용할 수 있음
    // 그럼 정의부에서, summarize_author() 메소드만 구현하면 됨
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
*/


// 매개변수로서의 트레이트
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 트레이트 바운드 구문을 사용하면, 두 매개변수가 같은 타입으로 강제됨
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify<T: Summary>(item1: &T, item2: &T) {}


// where 절을 사용하여, 여러 트레이트 바운드를 사용할 수 있음
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}


// 트레이트를 구현하는 타입을 반환
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


// impl Trait 문법을 쓴다고 해서 항상 동일한 타입을 반환하는 것은 아님
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}


// 트레이트 바운드를 사용해, 조건부 메서드 구현
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}