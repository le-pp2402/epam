pub struct NewsAritcle {
    pub headline: String, 
    pub localtion: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


pub trait Summary {
    fn summarize(&self) -> String;
}


impl Summary for NewsAritcle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.localtion)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}