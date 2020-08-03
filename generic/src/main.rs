// Function to find the largest number in a list of i32 numbers (not a generic)
//fn largest(list: &[i32]) -> i32 {
//    let mut largest = list[0];
//
//    for &item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}
// Let's make the above a generic function that can take in any type :)
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generic struct w/ method below
// can provide multiple types by  assigning like below
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// generic enum
enum Option<T> {
    Some(T),
    None,
}

// generic struct and building a method on it
struct Top<T> {
    x: T,
    y: T,
}

impl<T> Top<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// traits are a generic way to define behavior of a particular type and
// share that behavior with other types.
// Below is an example of trait implemented w/ two different methods
pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let number_list = vec![20, 50, 14, 100, 75];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 1000, 2000, 3000, 6000, 20];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'c', 'a'];
    let result = largest(&char_list);
    println!("The largest character is {}", result);
    
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 5, y: 4.0 };

    let t = Top { x: 5, y: 10 };
    println!("t.x = {}", t.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Calling the method w/ traits
    let tweet = Tweet {
        username: String::from("CoolH@ckers"),
        content: String::from(
            "well actually, let me be obnoxious and tell you",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet {}", tweet.summarize());
}
