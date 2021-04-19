fn main() {
    let mut s1 = "hello, String".to_string();
    println!("{}", s1);

    let mut s = String::new();
    s.push_str("გამარჯობა, სამყარო");
    s.push('!');
    println!("{}", s);

    let t1 = String::from("შენ");
    let t2 = String::from("დაგეძებ");
    let t3 = String::from("დილაა");

    let t = format!("{} {} {} თუ ღამეა", t1, t2, t3);
    println!("{}", t);

    for c in t.chars() {
        print!("{}-", c);
    }

    println!("");
    for b in t.bytes() {
        print!("{} ", b);
    }
}