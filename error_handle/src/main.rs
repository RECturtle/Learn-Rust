use std::fs::File;
use std::io::ErrorKind;
use std::io,
use std::io::Read;

fn main() {
    // Test to see the type File::open returns by setting f to expect u32
    // let f: u32 = File::open("Hello.txt");

    let f = File::open("Hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };
}

// Function that opens files and reads contents to string s (read_to_string
// is a function to do this)
// ? is like the match Ok/Err above -> returns ok if it's ok / err if an err
// BUT, ? utilizes from trait and converts errors from one type into another
// aka error type received by ? is passed through from function and
// returned as the error type defined at the front of the expression -
// For example - io::Error below
//
// Side note: it should be noted that fn main() has some restrictions on what types it 
// can return. Result<T, E> is an okay type.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s);

    Ok(s)

    // Quick way w/o all the error handling would be
    // fn read_username_from_file() -> Result<String, io::Error> {
    //      fs::read_to_string("hello.txt");
    // }
}
