fn main() {
    let i1: i32 = 5;
    let i2: i32 = i1;

    println!("i1: {i1}, i2: {i2}");

    // this will reach error because of Ownership rules in Rust
    // let s1: String = String::from("Rust");
    // let s2: String = s1;

    // println!("s1: {s1}, s2: {s2}");

    // to fix this, we can use the clone() method to create a deep copy of the string
    let s1: String = String::from("Rust");
    let s2: String = s1.clone();  // Reacts like Deep Copy

    println!("s1: {s1}, s2: {s2}");
}