fn main() {
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Refrence rules:
    /// 1. At any given time, you can have either one mutable reference or any number of immutable references.
    /// 2. References must always be valid.
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // let s = String::from("Rust");

    // let r1 = &s; // No problem
    // let r2 = &s; // No problem
    // println!("s: {}, r1: {}, r2: {}", s, r1, r2);

    // // let r3 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let mut s = String::from("Rust");
    // let r3 = &mut s; // No problem
    
    // r3.push_str(" is great!"); // Modifying s through r3

    // println!("r3: {}", r3); // This will print "Rust is great!"
    // println!("s: {}", s); // This will print "Rust is great!" as well, But retuns borrowing to `s` and compiler will not allow to use `r3` after this point because it is a mutable reference and we have already used it to modify `s`.
    // // println!("r3: {r3}");  // Error: cannot use `r3` because it was mutably borrowed
    
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // let mut s = String::from("Rust");

    // let r1 = &mut s; // No problem

    // s.push_str(" is Great!");  // Error: Modifying s directly while it is **borrowed** is not valid!

    // println!("r1: {}", r1); // This will print "Rust"

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // let mut s = String::from("Rust");

    // let r1 = &mut s; // No problem
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once at a time

    // // println!("s: {s}"); // This will ok and means that `s` is ended borrowing

    // println!("r1: {r1}"); // Error: cannot borrow `s` as mutable more than once at a time
    
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    // let mut s = String::from("Rust");

    // let r1 = &s; // No problem
    // let r2 = &s; // No problem
    // println!("s: {}, r1: {}, r2: {}", s, r1, r2);  // No problem ecause i can have mor than 1 immutable refrence to `s` at the same time but i cannot have mutable reference to `s` more than once at the same time

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let refrence_to_nothing = dangle();

}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}  // Here `s` goes out of scope and is dropped (with its refrence), so the reference returned by `dangle` would be pointing to invalid memory. Rust prevents this by not allowing us to compile code that returns a reference to a local variable.