fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;

    let sum = x + y.unwrap_or(0);

    println!("sum: {}", sum);

    println!("res: {}", x + z.unwrap_or(0));
}