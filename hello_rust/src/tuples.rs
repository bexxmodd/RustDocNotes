/*
* GRoup together values of different types
* Max 12 elements
*/

pub fn run() {

    let person: (&str, &str, i8, f32) = ("Beka", "Tbilisi", 32, 15.5);

    println!("{} is from {} and is {} years old with {} % body fat",
            person.0, person.1, person.2, person.3);
            
}