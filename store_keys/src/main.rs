use std::collections::HashMap;

fn main() {
    // Create Hash map and insert Blue: 10 and Yellow: 50
    // The keys and values need to be homogeneous
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Inserting same key w/ different value overwrites the value
    scores.insert(String::from("Yellow"), 100);

    // Check is a key has a value .or_insert(value) if it doesn't
    // If there is a value - return a mutable reference to value of entry key
    scores.entry(String::from("Orange")).or_insert(25);
    scores.entry(String::from("Blue)")).or_insert(50); // found value, won't change

    // Inserting types w/o Copy or reference will give ownership
    // to the hash map
    let field_name = String::from("Purple");
    let field_value = 20;

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // Here ownership of field_value
                                         // is given to scores

    // Create hashmap using iterator and collect method on vector of tuples
    let teams = vec![String::from("Green"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Rust can infer types so <_, _> is okay here
    let _teams: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Let's retrieve a value from the hash map
    let _score = scores.get("Blue"); 

    let team_name = String::from("Purple");
    let _new_score = map.get(&team_name); // grab using reference

    // iterate over hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // function to count number of words in a string
    let text = "hello world wonderful world";
    let mut mapper = HashMap::new();

    for word in text.split_whitespace() {
        let count = mapper.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", mapper);
}
