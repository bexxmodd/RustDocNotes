// Arrays are fixed list where elements are the same data types
// We can't add to the array but we can change individual values
// Arrays are stack allocated

pub fn run() {

    let mut numbers: [i32; 5] = [1, 3, 6, 7, 2];

    for n in &numbers {
        if n % 2 == 0 {
            println!("{} is an even number", n);
        }
    }

    // Get a single value
    println!("the second value in array is: {}", numbers[1]);

    // Re-assign value
    numbers[1] = 8;
    println!("{:?} has length of {}", numbers, numbers.len());

    // To check how many bytes array has allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Ger slice from an Array
    let sliced: &[i32] = &numbers[1..4];
    println!("{:?}", sliced);

}