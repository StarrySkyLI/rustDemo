use std::fmt::{Debug, Display};

pub trait Summary{
    //未实现，要实现
    fn summarize(&self)->String;
    // 已经实现，就不用自己实现，默认实现
    fn summarize_read(&self){
        println!("read more");

    }
    fn summarize_way(&self)->String{
        format!("(Read more from {}...",self.summarize())
    }
}
pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//trait：定义共享的行为
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

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}， {}", self.username, self.content)
    }
}
// trait做参数
pub fn notify1(item1: impl  Summary + Display){
    println!("Breaking news {}",item1.summarize())
}
pub fn notify<T:Summary+Display,U:Clone+Debug>(a:T,b:U)->String{
    format!("Breaking news {}", a.summarize())
}
pub fn notify2<T,U>(a:T,b:U)->String
where
    T: Summary+Display,
    U:Clone+Debug,
{
    format!("Breaking news {}", a.summarize())
}
// trait做返回值
pub fn notify3(flag : bool)-> impl Summary{
    if flag{
        NewsArticle{
            headline: "t1".to_string(),
            location: "t1".to_string(),
            author: "t1".to_string(),
            content: "t1".to_string(),
        }
    }else{
        //错误用法， trait做返回值，只能返回同样的
        // Tweet{
        //     username: "t2".to_string(),
        //     content: "t2".to_string(),
        //     reply: false,
        //     retweet: false,
        // }
    }
}