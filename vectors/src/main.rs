// Collections are all use heap - should be chosen carefully
// and are very useful
fn main() {
    // Vect<T> aka vector can store more than one value in a single data
    // structure
    // Should be noted that all values must be the same type
    // If initializing an empty vector, you have to add type annotation
    let v: Vec<i32> = Vec::new();

    // Rust will infer types automatically for the most part when you
    // provide values.
    // Also, vec! is a macro for convenience
    let v = vec![1, 2, 3];

    //accessing elements
    let third: &i32 = &v[2];
    println!("The third element in v is {}.", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }

    // Mutable vector
    let mut b = Vec::new();
    // pushing values into "b"
    b.push(5);
    b.push(6);
    b.push(7);
    b.push(8);

    // Iteration using a for loop
    for i in &b {
        println!("{}", i);
    }
    
    // Can also iterate over a mutable vector to change values
    let mut c = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in &mut c {
        *i += 50;
    }

    for i in &c {
        println!("{}", i)
    }

    // When you want to stored different types in a vector, you can use
    // an enum instead as they can hold values of different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
} // v and b out of scope here
