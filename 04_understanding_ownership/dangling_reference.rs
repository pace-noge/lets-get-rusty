fn main() {
    let ref_to_nothing = &dangle();
    println!("{}", ref_to_nothing);
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s;
} // s dropped here so the reference pointing to nothing