mod lib;
mod lib2;
mod lib3;

fn main() {
    // Q1 in lib
    let xs: [i32; 5] = [4, 6, 2, 5, 6];
    main_part2(xs);

}

fn main_part2(xs: [i32; 5]) {
    // let's make mutable reference
    for i in &xs {
        println!("{}", i);
    }

}
