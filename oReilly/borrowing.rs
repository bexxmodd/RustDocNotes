use std::collections::HashMap;

struct Person {
    name: String,
}

pub struct Player {
    score: i32
}

impl Player {
    pub fn set_score(&mut self, new_score: i32) {
        self.score = new_score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

pub struct Stats {
    hp: u8,
    sp: u8,
}

pub struct Monster {
    stats: Stats,
    friends: Vec<Friend>,
}

pub struct Friend {
    loyalty: u8,
}

impl Monster {
    pub fn final_break(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.stats.heal(friend.loyalty);
            println!("Healing for {}", friend.loyalty);
        }
    }
}

impl Stats {
    pub fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}


fn congrats(person: &Person) {
    println!("Congrats, {}!", person.name);
}

fn main() {
    let p = Person {
        name: String::from("Jake"),
    };

    let mut player1 = Player::new();
    player1.set_score(player1.score() + 3);

    println!("Player score is {:?}", player1.score());

    congrats(&p);
    println!("Can still be used {}", p.name);

    let mut list = vec![1,2,3];

    let first = list.first();
    let last = list.last();

    println!("first element is {:?} and second element is {:?}", first, last);

    *list.first_mut().expect("List was empty") += 1;

    let text = "testing some new and not new entry stuff for hash map for practice";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequecies: {:#?}", freqs);

}