// NewsArticle struct
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

// Trait implementation for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// Tweet struct
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Trait implementation for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}


// Trait for NewsArticle and Tweet structs
pub trait Summary {
    fn summarize(&self) -> String;
}



fn main() {

    let tweet: Tweet = Tweet {
        username: String::from("John Doe"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    }

    let article: NewsArticle = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling.")
    }

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}