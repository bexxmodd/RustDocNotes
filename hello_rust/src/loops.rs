pub fn run() {

    let mut count = 0;
    // eternal loop
    loop {
        count += 1;
        println!(">>>>> {}", count);

        if count == 10 {
            break;
        }
    }

    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    // For range
    for i in 0..100 {
        if i % 15 == 0 {
            println!("fIZZbUZZ");
        } else if i % 3 == 0 {
            println!("fIZZ");
        } else if i % 5 == 0 {
            println!("bUZZ");
        } else {
            println!("{}", count);
        }
        count += 1;
    }
}