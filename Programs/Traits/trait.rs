// NewsArticle struct
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

// Trait implementation for NewsArticle
impl Summary for NewsArticle {
fn summarize_author(&self) -> String {
    format!("@{}", self.author)
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}


// Trait for NewsArticle and Tweet structs
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default impl
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Traits as Parameters
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Generic limited to data type that implements the summary traits
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }


// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {

// }

// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {

// }


// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// }

// // using where clause for readability
// fn some_function<T, U>(t: &T, u: &U) -> i32 
//     where T: Display + Clone,
//           U: Clone + Debug
// {

// }

// Return traits
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horsebooks"),
//         content: String::from(
//             "of course, as you probably already know"
//         ),
//         reply: false,
//         retweet: false,
//     }
// }


// Logic on function
fn returns_summarizable(switch: bool) -> impl Summary {
    
    if switch {
            NewsArticle {
                headline: String::from("Penguin win the cup"),
                author: String::from("Iceburgh"),
                content: String::from("The Pittsburgh once again"),
            }

    } else {
            Tweet {
            username: String::from("horsebooks"),
            content: String::from(
                "of course, as you probably already know"
            ),
            reply: false,
            retweet: false,
        }
    }
}



fn main() {

    let tweet: Tweet = Tweet {
        username: String::from("John Doe"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    };

    let article: NewsArticle = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling.")
    };


    // println!("Tweet summary: {}", tweet.summarize());
    // println!("Article summary: {}", article.summarize());
    // println!("Tweet summary: {}", tweet.summarize_author());
    // println!("Article summary: {}", article.summarize_author());

    // notify(&article);

    println!("{}", returns_summarizable(false).summarize())
}

