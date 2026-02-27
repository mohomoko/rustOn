fn main() {
    let s1 = String::from("Hello, World!");

    /*
     * The string "Hello, World!" is stored in memory as a sequence of bytes.
     * 0  H
     * 1  e
     * 2  l
     * 3  l
     * 4  o
     * 5  ,
     * 6  (space)
     * 7  W
     * 8  o
     * 9  r
     * 10 l
     * 11 d
     * 12 !
     * 
     *  */

    let r1 =&s1;  // r1 is a reference to s1, it borrows the whole value of s1
    let slice = &s1[0..5]; // slice is a reference to a portion (0:'H' to 4:'o' and not include 5:',') of s1, it borrows a part of s1. the Type of slice is `&str`, which is a string slice, it is a reference to a portion of a string.
    println!("r1: {}, slice: {}", r1, slice);

    let slice= &s1[..5];
    println!("slice: {}", slice);

    let slice = &s1[7..13];
    println!("slice: {}", slice);

    let len = s1.len();
    let slice = &s1[7..len];
    println!("slice: {}", slice);

    let slice = &s1[7..];
    println!("slice: {}", slice);
    
    let slice = &s1[..];
    println!("slice: {}", slice);

    /////////////////////////////
    
    let s1 = String::from("Rust");  // Heap allocated string, s1 owns the string "Rust"
    let s2 = "Rust"; // String literal, s2 is a string slice, it is a reference to the string "Rust" stored in the binary of the program, it does not own the string.

    /////////////////////////
    // Types are same as s1:
    let s3 = "Rust".to_owned();
    let s4 = "Rust".to_string();

    let r1 = &s1;
    let slice: &str = &s1;

    println!("{slice}");

    ////////////////////////////
    let s1 = String::from("Hello, World!");
    let s1_first_5 = first_5_char(&s1);
    println!("{s1_first_5}");

    let s2 = "Hello, World!";
    let s2_first_5 = first_5_char(&s2); // error: mismatched types for commented `first_5_char` function
    println!("{s2_first_5}");

    let my_array = [1, 2, 3, 4, 5];

    let slice = &my_array[..3];

    println!("{:?}", slice);


}

// fn first_5_char(s: &String) -> &str {
//     &s[0..5]
// }

fn first_5_char(s: &str) -> &str {  /* It is accepting both `String` and `&str` because of **deref coercions** */
    &s[0..5]
}