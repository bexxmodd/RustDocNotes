/*
* Primitive str = Immutable fixed-length string somewhere in memory
* String = Growable, heap-allocated data structure
*           Use when you need to modify or own.
*/

pub fn run() {
    // Fixed type str
    let hello = "world";

    // Heap-allocated String
    let mut world = String::from("hello");

    println!("{}, {}!", world, hello);

    // Get length
    println!("world has length: {}", world.len());

    // Add (push) char to the String variable
    world.push('B');

    // Same push but for a str
    world.push_str("exx!");

    println!("pushed version: {}", world);

    // If contains a substring
    println!("check if contains ex: {}", world.contains("ex"));

    // Replace substring with another string
    let new_world = world.replace("Bexx", "Ann");
    println!("Now say {}", new_world);

    // Loop through string by whitespace
    let new_new_world = String::from("Welcome to New New York, year 3099");
    for word in new_new_world.split_whitespace() {
        println!("{}", word);
    }

    // Assertion testing
    let a = 'a';
    let b = 'b';
    assert_eq!(a, b);

}