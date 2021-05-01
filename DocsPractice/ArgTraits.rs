use traits::Summary;

// forcing both args to have same type and implement trait Summary
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize());
}

// If we want both args to have any type and implement trait Summary
pub fn update(item1: &impl Summary, item2: &impl Summary) {
    println!("First update : {}", item1.summarize());
    println!("Second update: {}", item2.summarize());
}

// arg that implements two traits
pub fn cool_notify(item: &(impl Summary + Display)) {
    println!("pass");
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}