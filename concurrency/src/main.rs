use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    
    // we have to move v to use in a multi-thread environment. Otherwise rust
    // doesn't know how long handle will take and this would cause an issue.
    // Ex: v is dropped, v is modified, etc. (all while it is used by thread.)
    // We move ownership from main to ensure it doesn't mess w/ anything while
    // the thread is running.
    let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
    });
    
    // putting join in different spots will have impact - see below
    // handle will run and main will wait until it's finished
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }
    // regular run and then main will wait for spawned thread to finish
    // handle.join().unwrap();
}
