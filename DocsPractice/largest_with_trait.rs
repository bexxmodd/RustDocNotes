fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn ref_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in 1..list.len() {
        if list[i] > *largest {
            largest = &list[i];
        }
    }
    return &largest;
}

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];

    let result = largest(&num_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'b', 'm', 'c', 'a'];
    let result = ref_largest(&char_list);
    println!("The largest char is {}", result);
}