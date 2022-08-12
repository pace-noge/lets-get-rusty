fn main() {
    fn a() {
        let x = "hello";
        let v = 22;
        b()
    }

    fn b() {
        let x = String::from("world");
    }

    { // s is not valid here, it's not declared yet
        let s = "hello"; // s is valid from this point forward
        // do stuff with s

    } // this scope is now over, and s is no longer valid.

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}", s1); // can't use it like this, the s1 value already moved do this instead

    let s2 = s1.clone();
    println!("{}", s1);
}
