fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    if number < 10 {
        println!("First condition is true");
    } else if number < 22 {
        println!("Second condition is true");
    } else {
        println!("Condition was false");
    }

    
}