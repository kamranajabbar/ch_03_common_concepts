pub fn run(){
    // Print to console
    println!("Hello Kamran");
    println!("My Number is {}", 10);

    //Basic formatting
    println!("My name is {} and i am from {}.", "Kamran Jabbar", "Pakistan");

    //Positional Arguments
    println!("{0} is living in {1} city and {0} is working in {2} since {3}", "KJ", "Karachi", "M3tech", "2016");

    //Named Arguments
    println!("{name} is working in {company_name} Since {year}", name="KJ", company_name="M3Technologies", year="2016");

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Kamran Jabbar"));

    //Basic Math
    println!("10 + 10 = {}", 10+10);
}