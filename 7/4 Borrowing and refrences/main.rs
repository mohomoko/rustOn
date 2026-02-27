// fn main() {
//     let s = String::from("Rust");
//     do_something_else(&s); // borrow `s` by passing a reference to it (refrence borrowing)
//     println!("s: {s}");
// }

// fn do_something_else(some_string: &String) {  // some_string is a reference to a String, so it does not take ownership of the String
//     println!("Doing something else...");
//     println!("some_string: {some_string}");
// }  /* some_string goes out of scope here, but since it does not have ownership of the String, nothing happens. The String is not dropped. */

fn main() {
    // Running
    // let mut s1 = String::from("Hello");
    // s1.push_str(", World!");
    // println!("s1: {s1}");

    let mut s1 = String::from("Hello");
    do_something_and_change_mutable(&mut s1); // borrow `s1` mutably by passing a mutable reference to it (mutable reference borrowing)
    println!("s1: {s1}");
    do_something(&s1); // we can still borrow `s1` immutably after borrowing it mutably, as long as the mutable borrow is no longer active
}

fn do_something_and_change_mutable(some_string: &mut String) {  // some_string is a mutable reference to a String, so it does not take ownership of the String, but it can modify the String through the mutable reference
    // some_string.push_str(", World!"); // we can modify the String through the mutable reference
    println!("some_string: {some_string}");
}  /* some_string goes out of scope here, but since it does not have ownership of the String, nothing happens. The String is not dropped. */

fn do_something(some_string: &String) {  // some_string is a reference to a String, so it does not take ownership of the String
    println!("Doing something...");
    println!("some_string: {some_string}");
}  /* some_string goes out of scope here, but since it does not have ownership of the String, nothing happens. The String is not dropped. */