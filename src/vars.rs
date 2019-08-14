pub fn run(){
    let name = "Kamran Jabbar";
    let mut age = 30;
    println!("My name is {} and i am {} today", name, age);


    age = 31;
    println!("My name is {} and i will be {} tomorrow", name, age);

    //Define Constant
    const ID: i32 = 0001;
    println!("My ID: {}", ID);

    //Assign Multiple Vars
    let (name, age) = ("Kamran Jabbar", 30);
    println!("My name is {} and i am {} years old", name, age);
}