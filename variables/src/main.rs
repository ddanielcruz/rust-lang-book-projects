// Constants are declared using the `const` keyword, they can be declared in any scope and they work like C# constants
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Variables are immutable by default
    let mut x = 5;
    x += 1;
    println!("1. The value of x is: {x}");
    
    // Shadowing a variable by reusing its name
    let x = 10;
    println!("2. The value of shadowed x is: {x}");

    {
        let x = x * 2;
        println!("3. The value of shadowed x in the inner scope is: {x}");
    }

    println!("4. The value of shadowed x is: {x}");

    let arr = [3; 5];
    println!("5. The value of arr is: {:?}", arr);
}
