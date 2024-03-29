pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking new! {}", item.summarize_author());
}

fn main() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(), 
        content: "Rust棒极了!".to_string()
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string()
    };
    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
    notify(&weibo);
    notify(&post);
}

