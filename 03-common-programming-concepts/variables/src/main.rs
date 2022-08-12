fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("Subscriber {}", SUBSCRIBER_COUNT);


    // shadowing
    let y = 4;
    println!("The value of y is: {}", y);

    // shadowing, redeclare the variable using let
    let y = "four";
    println!("The value of y is: {}", y);

    // scalar data type
    //  Integers
    //  Floating-point numbers
    //  Booleans
    //  Character


    // Integers
    let a = 98_222;  // Decimal
    let b = 0xff;  // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 Only);

    let f: u8 = 255;

    // Floating-point numbers
    let g = 2.0;
    let h: f32 = 3.0;

    // addition
    let sum = 5 + 10;
    // Substraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f = false;

    // Character
    let c = 'z';

    // compound type (group of scalar type)

    let tup = ("Get Rusty", 100_000);
    let (channel, sub_count) = tup;


    let sub_count = tup.1;

    // array
    let error_codes = [200, 404, 500];
    let not_cound = error_codes[1];

    let byte = [0; 8];
    
    my_function();
    another_function(20, 5);

    println!("expression {}", expression(5, 4));

}


fn my_function() {
    println!("Another function");
}

fn another_function(x: i32, y: i32) {
    println!("The value of X: {}", x);
    println!("The value of Y: {}", y);
}

fn expression(x: i32, y: i32) -> i32 {
    x + y
}