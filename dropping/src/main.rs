// implement a Custom Pointer struct
struct CustomPointer {
    data: String,
}

// Create implementation for Drop for CustomPointer and create drop method.
// The drop method is the code called when instances are dropped from scope.
impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping CustomPointer with data '{}'!", self.data);
    }
}

fn main() {
    let c = CustomPointer {
        data: String::from("my stuff"),
    };
    let d = CustomPointer {
        data: String::from("other stuff"),
    };
    println!("CustomPointer created.");
    // We can pass a variable as an argument in drop to force an instance to be
    // dropped early.
    drop(d);
    println!("CustomPointer dropped d (other stuff) before the end of main.")
} // Drop code gets called here when c and d go out of scope
