// Time to try out some functions
// Pass variables into function w/ comma separation
fn main() {
    // Call another function and pass 5, 6
    another_function(5, 6);

    // Example of scope - first z is outside of block and local z is called
    // in the block below and w is set to the result
    let _z = 6;

    let w = {
        let z = 3;
        z + 1
    };

    println!("The value of w is: {}.", w);

    // Call five and assign u to result
    let u = five();
    println!("The value of u is: {}.", u);
    
    // Call plug_one and assign result to a
    let a = plus_one(5);
    println!("The value of a is: {}.", a);
}

// Establish type w/ function - works like other langs
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);
}

// Simple function that returns 5
// Since 5 is an expression and not a statement there is no
// need for a ;
fn five() -> i32 {
    5
}

// Function that takes a value "a" and returns +1
fn plus_one(a: i32) -> i32 {
    return a + 1; // ; here because it is a statement (direction) & expression
}
