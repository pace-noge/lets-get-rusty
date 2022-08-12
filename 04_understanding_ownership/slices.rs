fn main() {
    // let s = String::from("Hello world");

    // let hello = &s[0..5]; // &s[..5]
    // let world = &s[6..1]; // &s[..]

    // let word = first_word(&s);
    // println!("{}", word);

    let mut s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(&s2);
    println!("{}", word);

    let word = first_word(&s);
    println!("{}", word);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}