// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn fun(){

    //variables
    let mut age =  18;
    println!("{}." , age);
    age = 19;
    // would not be possible to reassign if it was not 'mut.
    println!("{}.", age);

    // const : must define a type.
    const ID: i32 = 001;
    println!("ID = {}", ID);

    // multiple vars
    let (day, date) = ("Sunday", 22);
    println!("{} and {}", day, date);
}