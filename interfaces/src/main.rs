
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Source: {}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", 
            self.headline, 
            self.author, 
            self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}",
            self.username,
            self.content)
    }
}

pub fn notify<T: Summary>(item: &T, news_or_tweet: bool) {
    if news_or_tweet {
        println!("Breaking: {}", item.summarize());
    }
    else {
        println!("1 new tweet: {}", item.summarize());
    };
}

fn main() {
    let tweet = Tweet {
        username: String::from("XXX_420_XXX"),
        content: String::from("I love Rust!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("CS:GO 2 Announced"),
        location: String::from("Valve"),
        author: String::from("counter-strike.net"),
        content: String::from("Counter-Strike 2 is the largest technical 
                leap forward in Counter-Strike's history, 
                ensuring new features and updates for years to come..."),
    };

    notify(&tweet, false);
    notify(&article, true);
}