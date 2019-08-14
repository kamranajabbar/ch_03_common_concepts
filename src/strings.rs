pub fn run() {
    let mut hello = String::from("Hello ");

    println!("{}", hello);

    //Push Char
    hello.push('K');

    //Push String
    hello.push_str("amran");

    //Get length
    println!("Length : {}", hello.len());

    //Get Capacity
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Is empty: {}", hello.is_empty());

    //Contains
    println!("Contains : {}", hello.contains("Kamran"));

    //Replace
    println!("Replace : {}", hello.replace("Hello", "Hi"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("--- {} ---", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
}
