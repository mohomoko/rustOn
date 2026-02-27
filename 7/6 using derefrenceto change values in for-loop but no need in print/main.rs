fn main() {
    // using dereferencing to assign value to a mutable from reference 
    // let mut s = String::from("Rust");
    // println!("{}", s);

    // s = String::from("ferris");
    // println!("{}", s);

    // let r1 = &mut s;
    // // r1.push_str(" m");
    // // r1 = String::from("Ferris m");  // Error: cannat assign to `r1` because it is borrowed
    // *r1 = String::from("Ferris m");  // No Problem, because used Derefrencing to assign value to `s` through `r1`
    // println!("{}", r1);
    // println!("{}", *r1);  // no need to dereference `r1` to print its value, because `println!` can handle references

    //////////////////////////////////////////////////
    
    let mut scores: [i32; 4] = [0; 4];

    for element in scores {
        println!("{}", element);
    }
    
    println!("------------------");
    
    for element in &scores {
        println!("{}", element);
    }
    
    println!("------------------");
    
    for element in &mut scores {
        // element = 10;  // Error: cannot assign to `element` because it is a `Copy` type
        // element = 10;  // Error: cannot assign to `element` because it is a `&i32`
        // element = 10;  // Error: its refrence to an integer, so we can assign value to it
        *element = 10;  // No Problem, because used Derefrencing to assign value to `scores` through `element`
    }
    
    for element in scores {
        println!("{}", element);
    }
    println!("------------------");
}