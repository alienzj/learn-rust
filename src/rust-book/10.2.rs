pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Paper {
    pub author: String,
    pub title: String,
    pub journal: String,
    pub describtion: String,
}

impl Summary for Paper {}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_full<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item1: impl Summary, item2: impl Summary)
// pub fn notify<T: Summary>(item1: T, item2: T)

// pub fn notify(item: impl Summary + Display)
// pub fn notify<T: Summary + Display>(item: T)



fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let paper = Paper {
        author: String::from("LMN"),
        title: String::from("MesKit"),
        journal: String::from("Bioinformatics"),
        describtion: String::from("Multi-region exome sequencing analysis tool kit"),
    };
    println!("New paper available! {}", paper.summarize());
}
