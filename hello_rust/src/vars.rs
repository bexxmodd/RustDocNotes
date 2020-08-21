pub fn run() {
    let name = "Bexx";
    let mut age = 32;
    println!("My name is {} and I am {} years old", name, age);

    age += 1;
    println!("My name is {} and I am {} years old", name, age);

    // Define Constant
    const ID: i32 = 333;
    println!("My id is {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("beka", 40);
    println!("My name is {}, my age is {}", my_name, my_age);
    
}