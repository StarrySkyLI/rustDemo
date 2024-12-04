use trait_demo::{notify3, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Username"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };
    println!("new tweet: {}", tweet.summarize());
    tweet.summarize_read();
    println!("new tweet: {:?}", tweet.summarize_way());
    let t3 = notify3(true);
    println!("{}", t3)
}
