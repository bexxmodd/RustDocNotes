fn main() {
    println!("Now we'll calculate square value of an x");
    println!("square of 4 is {}", square_function(4));

    println!("72 degree Fahrenheit is {} degree Celsius", convert_degree(72.0));
}


fn square_function(x: i32) -> i32 {
    x * x
}

fn convert_degree(x: f32) -> f32 {
    (x - 32.0) * (5.0/9.0)
}