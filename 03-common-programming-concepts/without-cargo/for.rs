fn main() {
    let a = [10, 20, 30, 40, 50];

    for el in a.iter() {
        println!("element value {}", el);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}