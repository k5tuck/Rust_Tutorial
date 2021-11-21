pub fn run(){
    // Print to console
    println!("Hello Rust World!");

    // Basic Formatting
    println!("{} is from {}", "Kevin", "Georgia");
    
    // Positional Arguements
    println!("{1} going to the {0}", "beach", "I'm");
    
    // Named Arguements
    println!("{name} likes to play the {activity}", name = "Kevin", activity = "PS5");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug Trait
    print!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("\n{} + {} = {}", 9, 9, 9+9)

}