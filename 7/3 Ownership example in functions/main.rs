// fn main() {
//     let i: i32 = 5;
//     do_something(i);
//     println!("i: {i}");
    
//     let s = String::from("Rust");
//     do_something_else(s);
//     /* so `s` moved to some_string and some_string is dropped! so `s` is not availabe! */
//     println!("s: {s}");  // and we get an error here because `s` is not available anymore
// }

// fn do_something(some_integer:i32 ) {  /* some_integer makes copy */
//     println!("Doing something...");
//     println!("some_integer: {some_integer}");
// }

// fn do_something_else(some_string: String) {  /* some_string moves/takes ownership */
//     println!("Doing something else...");
//     println!("some_string: {some_string}");
// }  /* some_string goes out of scope and is dropped here so it cannot be used anymore */

fn main() {
    let mut s1 = gives_ownership();  /* gives_ownership moves its return value into s1 */
    println!("s1: {s1}");
    s1 = String::from("Ferris");
    println!("s1: {s1}");
    let s2 = takes_and_gives_back(s1);  /* s1 is moved into takes_and_gives_back, which also moves its return value into s2 */
    println!("s2: {s2}");
    // println!("s1: {s1}");  /* we get an error here because s1 is not available anymore */
}

fn gives_ownership() -> String {  /* gives_ownership will move its return value to the caller */
    let mut some_string = String::from("Rust");
    some_string  /* some_string is returned and moves out to the caller */
}

fn takes_and_gives_back(mut some_string: String) -> String {  /* takes_and_gives_back will take a String and return one */
    some_string = String::from("Rustacean");
    some_string  /* some_string is returned and moves out to the caller */
}