fn main() {
    // Basic slice
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3];
    println!("Slice of a: {}, {}.", slice[0], slice[1]);
    
    // Now let's try string slices
    let s = String::from("hello world");

    // hello could also be sliced &s[..5]; and it would work.
    // world could also be sliced &s[6..]; and it would work.
    let _hello = &s[0..5];
    let world = &s[6..11];

    println!("{}", world);

    // Let's try passing values to a function.
    // Passing an index makes this API more general and reliable
    let word = first_word(&s[..]);
    println!("Before first space: '{}'.", word);

    // Try with a string literal
    let s_literal = "hello world";
    let word_two = first_word(&s_literal[..]);
    println!("Literal: Before first space: '{}'", word_two);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
