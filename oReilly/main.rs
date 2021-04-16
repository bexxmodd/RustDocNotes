fn main() {
    let mut primes = vec![2, 3, 5, 7];
    primes.push(11);

    let product = primes.iter().product::<i32>();
    println!("Product = {}", product);

    let v: Vec<u32> = (0..32).collect();
    for i in v {
        print!("{} ", i);
    }
    println!("\n----------");

    let mut v1 = Vec::<u32>::with_capacity(20);
    v1.push(12);
    v1.push(15);
    println!("Capacity: {}", v1.capacity());
    println!("Len: {}", v1.len());
}