fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;
    
    // move changes ownership to closure equal_to_x so the below won't work
    // println!("printing x won't work {}", x);

    let y = vec![1, 2, 3];

    //iterator time
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    assert!(equal_to_x(y));

    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v3, vec![2, 3, 4]);
}
