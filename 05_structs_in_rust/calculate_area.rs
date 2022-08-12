#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect)
    );

    let rect2 = Rectangle {
        height: 30,
        width: 50,
    };

    println!("rect: {:#?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect2)
    );

    println!("height: {}", rect2.height);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// pass the rectangle as reference becuse we don't want to take the rectangle ownership
fn area_struct(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}