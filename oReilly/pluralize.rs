//! pluralize words

fn main() {

    let s = String::from("book");
    let ss = pluralize(s.clone());
    
    
    println!("I have one {}", s);
    println!("I have many {}", ss)

}

fn pluralize(s: &str) -> String {
    let mut pl = s
}