// Tuples group togather values of defferent types
// Max 12 element can carry

pub fn run(){
    let person: (&str, &str, i16) = ("Kamran Jabbar", "Karachi", 1988);

    println!("{} is living in {} since {}", person.0, person.1, person.2);
}