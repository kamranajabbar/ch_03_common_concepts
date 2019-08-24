pub fn run(){
    let mut number: [i32; 3] = [1,2,3];

    // Print whole array
    println!("{:?}", number);

    // Get single value
    println!("Single value of array: {}", number[2]);

    // Re-assign value
    number[2] = 20;
    println!("After reassignment, new array will be: {:?}", number);

    // Get array length
    println!("Array length: {}", number.len());

    // Array are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&number));
}