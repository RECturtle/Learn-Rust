fn main() {
    // Basic counter in Rust
    let mut counter = 0;

    let result = loop {
        println!("{} again!", counter);
        counter += 1;

        if counter == 10 {
            break counter; // Can perform operation here like counter * 2
        }
    };

    println!("The loop ran {} times.", result);

    // Fun countdown program (think this one might surprise the astronauts lol)
    println!("\n=== COUNTDOWN BEGINNING ===");
    let mut countdown = 10;

    while countdown != 0 {
        println!("{}!", countdown);
        countdown -= 1;
    }

    println!("=== LIFTOFF ===\n");

    // Looping through an array "a"
    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut index = 0;

    while index < a.len() {
        println!("The value of index {} is: {}", index, a[index]);

        index += 1;
    }

    // Optimal version of the array looping above
    // Utilized .enumerate() on it to get the index and num
    println!("\nOptimal version of the array looping:");
    for (index, num) in a.iter().enumerate() {
        println!("The value of index {} is: {}", index, num)
    }

    // Reverse the looping
    println!("\nPrint a range of numbers in reverse:");
    for num in (1..11).rev() {
        println!("Pop: {}", num);
    }
}
