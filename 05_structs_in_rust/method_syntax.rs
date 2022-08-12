#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let rect = Rectangle {
        height: 30,
        width: 50,
    };

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );

    let rect1 = Rectangle {
        height: 20,
        width: 40
    };

    println!("Rect can hold rect1 {}", rect.can_hold(&rect1));

    let rect2 = Rectangle {
        height: 40,
        width: 60,
    };

    println!("rect can hold rect2 {}", rect.can_hold(&rect2));

}