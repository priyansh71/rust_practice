// Traits allow us to define a common set of functions for different types.

pub struct Article {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn find_author(&self) -> String {
        format!("{}", self.author)
    }
} 

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn find_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// for every type that implements the `Summary` trait, we can initialize 
//the Summary function here or define a default one here
pub trait Summary {
    fn find_author(&self) -> String; 

    fn summarize(&self) -> String {                         // default implementation
        format!("Read more from ... {}", self.find_author())
    }
}

pub fn fun(){
    let tweet = Tweet {
        username: String::from("horse"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = Article {
        title: String::from("Penguins"),
        author: String::from("@horse_ebooks"),
        content: String::from("The birds are not very large."),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
}