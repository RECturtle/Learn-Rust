struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageNew {
    ChangeColor(Color),
}

struct Points {
    x: i32,
    y: i32,
    z: i32,
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y param: {}", y);
}

fn main() {
    let origin = Points { x: 0, y: 0, z: 0 };
    
    match origin {
        Points { x, .. } => println!("x is {}", x),
    }
    // Won't compile because let only accepts irrefutable patterns
    // and some_option_value = None
    // let Some(x) = some_option_value
    
    // This will work as if let will skip over this block since Some(x)
    // cannot equal None
    // if let Some(x) = some_option_value {
    //    println!("{}", x);
    //}

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let y = 5;

    match y {
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }

    let z = 'c';
    match z {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let a = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    // let msg = Message::Quit;
    // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Write(String::from("Hi there followers!"));
    let msg = Message::Move { x: 10, y: 10 };

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    let new_msg = MessageNew::ChangeColor(Color::Hsv(0, 160, 255));

    match new_msg {
        MessageNew::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        MessageNew::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cannot overwrite an existing customized value.");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let d = Some(5);
    let e = 10;

    match d {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, d = {:?}", d),
    }
    println!("at the end: d = {:?}, e = {}", d, e);
}
