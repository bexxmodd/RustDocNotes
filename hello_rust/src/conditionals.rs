pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;


    if age >= 21 && check_id {
        println!("Bartender: what would you like to drink");
    } else if age < 21 && check_id {
        println!("No drinks for you");
    } else {
        println!("I'll need to see your ID");
    }

    // Short-hand if
    let is_of_age = if age >= 21 { true } else { false };

    println!("is of age: {}", is_of_age);
}