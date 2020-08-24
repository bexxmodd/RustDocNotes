fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    // test end_and_print
    let array: [i32; 7] = [3, 6, 2, 6, 22, 98, 2]; 
    end_and_print(&array);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn square_function(x: i32) -> i32 {
    x * x
}

fn convert_degree(x: f32) -> f32 {
    (x - 32.0) * (5.0/9.0)
}

fn end_and_print(array: &[i32]) {
    let size = array.len();

    let mut i = 0;
    while i < size - 1 {
        println!("Let's go in : {}", array[i]);
        i += 1;
    }
}

fn feed_and_print(slice: &[i32], n: i32) {
    for i in 0..n {
        assert_eq!(slice.len(), n) {
            
        }
    }
}