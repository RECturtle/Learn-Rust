fn main() {
    // Create String s1 and pass a reference "&" to s1 to pass it to function
    // without having the function take ownership
    // x stays in scope since a reference was passed and ownership stays
    let s1 = String::from("hello");
    let x = calculate_length(&s1);

    println!("The length of {} is {}.", s1, x);

    // A function can modify what the var it borrows if it is mutable
    // aka mutable reference
    let mut s2 = String::from("hello");
    change(&mut s2);

    // Unlike mutable references - multiple immutable references are allowed
    // in a single scope
    let s3 = String::from("hello");
    let r1 = &s3; // r1 is passed immutable reference to s3
    let r2 = &s3; // r2 is passed immutable reference to s3
    // if both &s3 were &mut s3 -> compile error would pop
    println!("{} immutable {}", r1, r2);
}

// Required to call out type of s is a reference to a String with "&"
// This is called borrowing in Rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope here and is dropped, but return is kept in reference

// change borrows mutable s2 (aka reference to s2) and can change it
// Important note: can only have 1 mutable borrow in a scope/block
// e.g. if I borrowed two mutable Strings, it would throw an error
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
