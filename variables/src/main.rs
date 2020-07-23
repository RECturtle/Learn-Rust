fn main() {
    // Create a reg immutable variable
    let x = 5;
    println!("The value of x is {}.", x);
    println!("x is immutable.\n");
    
    // Create a mutable variable and change it
    let mut y = 6;
    println!("The value of y is {}.", y);
    y = 7;
    println!("But, y is mutable by including mut.");
    println!("y = {}\n", y);

    // Shadow x from above
    println!("x is not a const and can be shadowed.");
    let x = x + 1;
    println!("x + 1:");
    println!("x = {}\n", x);
    let x = x * 2;
    println!("x * 2:");
    println!("x = {}", x);

    // Constant
    const MAX_POINTS: u32 = 100000;
    println!("Created constant - can never change.");
    println!("MAX_POINTS = {}", MAX_POINTS);

    // Floating points
    // Fun fact: prefix variable with _ if unused on purpose
    let _z = 2.0;
    let _w: f32 = 3.0;

    // Boolean
    let _t = true;
    let _u: bool = false;

    // Create a tuple and access a certain index
    // You can also assign type let s: (i32, f64, u8) = ()
    let s = (500, 6.4, 2);
    println!("Created a tuple 's'.");
    println!("s.0 = {}, s.1 = {}, s.2 = {}", s.0, s.1, s.2);
    
    // You can also destructure a tuple
    let (a, b, c) = s;
    println!("Destruct tuple s to vars: a, b, c.");
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Create an array (fix length, single type)
    let months = ["January", "February", "March", "April", "May","June",
    "July", "August", "September", "November", "December"];
    println!("months[0] = {}.", months[0]);
}
