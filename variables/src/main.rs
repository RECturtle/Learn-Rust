fn main() {
    let x = 5;
    println!("The value of x is {}.", x);
    println!("x is immutable.");
    
    let mut y = 6;
    println!("The value of y is {}.", y);
    y = 7;
    println!("But, y is mutable by including mut.");
    println!("y = {}", y);
}
