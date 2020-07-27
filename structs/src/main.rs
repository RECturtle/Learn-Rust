// This file doesn't do anything, just informational and will compile
// with never read/used warnings.

// Define a struct
// Inside a struct are fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Create an instance of the User struct
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("niceUsername"),
        active: true,
        sign_in_count: 1,
    };
    
    // Update email. Notice the '.' notation to access fields
    user1.email = String::from("changed@example.com");

    // Rust has nifty shorthand for creating a new class and calling out what
    // is passed from another struct
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1 // takes the rest of the fields from user1
    };

    // Each individual tuple struct has it's own type
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Cannot pass Color to Point or vice versa as they are different types
}

// Build a user function
// Email and username are shorthand since they are passed
// no sense doing email: email if not necessary.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
