// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let array1 = [1, 3, 5, 2];
    let array2 = array1;

    println!("Array1: {:?}; Array2: {:?}", array1, array2);

    // Non Primitve Vector
    let vec1 = vec![3, 5, 2, 1];
    let vec2 = &vec1;
    println!("Vector1: {:?}; Vector2: {:?}", vec1, vec2);
}