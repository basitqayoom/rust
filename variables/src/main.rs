fn main() {
    // let and mutability
    println!("Let & Mutability");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    println!("Constants");
    const TIME: u32 = 30 * 10;
    println!("The const time value is {}", TIME);

    // Shadowing
    println!("Shadowing");

    let x = 5;
    let x = x + 1; // x = 6
    {
        let x = x * 2; // x = 6 * 2 = 12
        println!("The value of x in the inner scope is: {x}"); // -> 12
    }
    println!("The value of x is: {x}"); // -> 6

    let name = "Hello";
    println!("The value of name is {}", name);
    let name = 100;
    println!("The value of name is {}", name);
}
