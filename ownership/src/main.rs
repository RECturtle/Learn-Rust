fn main() {
    // string literal (immutable)
    // hardcoded and known at compile time (stack)
    let _s = "hello";
    
    // String type (mutable and manually allocated)
    // Can be updated and is allocated in heap
    // push_str to append onto x
    let mut x = String::from("hello");
    x.push_str(", world!");
    println!("{}", x);
    takes_ownership(x);
    // Can't do the following since x was moved to takes_ownership
    // println!("{}", x);
    
    // Create String and show copy/clone methods in Rust
    let s1 = String::from("hello");
    // If this was s2 = s1, s1 would move to s2 and no longer be valid
    // Below is the appropriate way to to copy a heap var
    let s2 = s1.clone();
    println!("s1 = {} : s2 = {}", s1, s2);

    // ints are small and easy to copy...
    // Therefore, they don't need clone and w is not moved to z
    let w = 3;
    let z = w;
    println!("w = {} : z = {}", w, z );

    makes_copy(w);
    //i32 is able to Copy so we can still use w
    println!("Look w is still available:");
    println!("w = {}", w);
}
// variables above fall out of scope and are no longer valid
// Rust auto calls the "drop" function

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Copy of w = {}", some_integer);
}
