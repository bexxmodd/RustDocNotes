fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup = (300, 6.3, 3);
    let (x, y, z) = tup;
    println!("The value of x is {} and the value of y is {}", x, y);

    let by_dot = tup.1;
    println!("{} was Picked with dot method", by_dot);

    let a: [u32; 4] = [3, 2, 5, 23];

    let first_a = a[0];
    println!("Here's the value from the array -> {}", first_a);
}
