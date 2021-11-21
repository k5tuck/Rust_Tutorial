// Variables hold primitive data or references to data
// Variables are imutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Kevin";
    println!("My name is {}", name);
    
    // Make a variable Mutable
    let mut age = 31;
    println!("I am {} years old", age);
    
    age = 32;
    println!("I turn {} next year", age);

    // Define Constant
    // const will normally be all Uppercase and the type must be defined
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multiple vars
    let (my_name, my_age) = ("Kevin", 31);
    println!("{} is {} years old", my_name, my_age);
}