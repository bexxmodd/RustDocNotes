pub fn run() {
    greeting("Hello", "World");
    
    // Bind function values to variables
    let sum = add(4, 3);
    println!("sum: {}", sum);

    // Closure
    // with closures you can use outside variable
    let n3: i32 = 22;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(11, 32));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

