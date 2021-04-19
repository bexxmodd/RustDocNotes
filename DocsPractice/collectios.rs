enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

    let mut v2 = Vec::new();
    v2.push(2);
    v2.push(31);
    v2.push(-23);
    v2.push(89);

    match v2.get(1) {
        Some(second) => println!("The second element is {}", second),
        None => println!("Item not found"),
    }

    let third = v2[2];
    v2.push(90);
    println!("thir element is {}", third);

    for i in &mut v2 {
        *i += 32 >> 2;
    }

    for i in &v2 {
        print!("{}, ", i);
    }
    

    // Using an enum plus a match expression means that Rust will
    // ensure at compile time that every possible case is handled
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("\nLength of vector: {}", row.len());
    
}