struct Person {
    name: String,
}

fn congrats(person: &Person) {
    println!("Congrats, {}!", person.name);
}

fn main() {
    let p = Person {
        name: String::from("Jake"),
    };

    congrats(&p);
    println!("Can still be used {}", p.name);

}