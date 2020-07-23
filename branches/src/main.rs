fn main() {
    // Standard if/else if/else structure
    let number = 7;

    if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is even")
    } else {
        println!("Number is not even or divisible by 3.")
    }

    // If is an expression and can be used to the right of a let
    // if/else format like cpp if (condition) ? true:false
    // if/if else/else arms need to evaluate to the same type 
    let condition = true;
    let value = if condition { 5 } else { 6 };

    println!("The value of value is: {}", value);
}
