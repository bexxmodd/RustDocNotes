
// Vectors are resizable arrays
// to define a vector use Vec<dataType> = vec!


use std::mem;

pub fn run() {

    let mut numbers: Vec<i32> = vec![32, 11, 33, 4, 5, -2, 99];

    // Loop thorugh the Vector
    for i in numbers.iter() {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
    numbers.push(3);
    numbers.push(73);
    numbers.pop();

    println!("{:?}", numbers);
    // Loop and mutate Vector
    for i in numbers.iter_mut() {
        *i *= 2;
    }

    println!("{:?}", numbers);
}