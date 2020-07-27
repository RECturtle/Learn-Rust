// struct to hold Rectangle fields
struct Rectangle {
    width: u32,
    height: u32,
}

// Method to hold functionality
// Can also define struct within a method -> fn square to create square
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Create 3 rectangles
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    // Area of rect1
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    // Check if rect2 and 3 can fit in rect 1
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // Create sq from Rectangle method and print area
    let sq = Rectangle::square(3);
    println!("Area of sq = {}", sq.area());
}
