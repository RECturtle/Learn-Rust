// Create an enum for the two types of ip address formats
// Create IpAddr as a custom type that can be used in code
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Create new enum Message
// Can be better than struct since all can be their own types
// Doing this w/ structs would end with 4 different structs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Can define methods on enums like structs
impl Message {
    fn call(&self) {

    }
}

// Null does not exist in rust; however, Option<T> is a built-in library
// that can encode the concept of a value being present or absent
enum Option<T>{
    Some(T),
    None,
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();

    // Couple examples of using Option<T> Some variant
    let some_number = Some(5);
    let some_string = Some("a string");
    
    // We use <i32> because to callout the type that's missing
    // None needs this to know what it is looking for
    // This throws an error about not matching types, but is from the 
    // book so will leave in here for now.
    let absent_number: Option<u32> = None;
}

// Create a function that the type IpAddr can be passed to
fn route(ip_kind: IpAddr) {

}
