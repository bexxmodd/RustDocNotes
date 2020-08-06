fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for i in (1..5).rev() {
        println!("Reversed value becomes: {}", i)
    }
}