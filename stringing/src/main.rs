fn main() {
    // create a string
    let s = String::new();

    // create a new string in a different way
    let data = "initializing string";
    let s = data.to_string();
    println!("{}", s);

    // create a new string and push onto it
    let mut n = "initial content".to_string();
    n.push_str("bar");
    n.push('1');
    println!("{}", n);

    // combine 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 was moved here and can't be used after
    println!("{}", s3);
    
    // neat formatting trick
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s7);
    
    // grabbing a slice
    let s8 = &s7[0..4];
    println!("{}", s8);
    
    // ways to loop through chars
    for c in "abcde023".chars() {
        println!("{}", c);
    }

    for b in "abcde023".bytes() {
        println!("{}", b);
    }
}
