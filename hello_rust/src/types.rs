/* 
* Integers: u8, i8, ... u128, i128 (number of bits they take in memory)
* Floats: f32, f64
* Boolean (bool)
* Characters (char)
*/

pub fn run() {
    println!("Max i16: {}", std::i16::MAX);

    // char is a unicode variable type and they are closed with single quotes
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{} very nice emoji: {}", a1, face);
}