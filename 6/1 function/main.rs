// fn main() {

//     do_something();
// }

// fn do_something() {
//     // Code to do something
// }

// fn main() {
//     println!("begin");
    
//     say_hello(String::from("moho"));  // Call method
    
//     let name = String::from("mohomoko");
//     say_hello(name);
//     say_hello(String::from("moko"));
    
//     println!("end");
// }

// fn say_hello(name : String) {
//     println!("Hi {}!", name);
// }

// fn main() {
//     println!("begin");
    
//     say_hello(String::from("moho"), String::from("value"));  // Call method
    
//     let f_name = String::from("eren");
//     let l_name = String::from("Yeager");
//     say_hello(f_name, l_name);  // f_name and l_name are arguments, first_name and last_name are parameters
//     say_hello(String::from("moho"), String::from("moko"));
    
//     println!("end");
// }

// fn say_hello(first_name : String, last_name : String) /* function signiture: fn say_hello(first_name : String, last_name : String) */ {
//     println!("Hi {} {}!", first_name, last_name);
// }

fn main() {
    println!("begin");
    
    say_hello(String::from("moho"), String::from("value"));
    
    let f_name = String::from("eren");
    let l_name = String::from("Yeager");
    say_hello(f_name, l_name);
    say_hello(String::from("moho"), String::from("moko"));
    
    println!("end");
}

fn say_hello(mut first_name : String, mut last_name : String) /* Mutable dont need to defenition */ {
    first_name = first_name.to_uppercase();
    last_name = last_name.to_uppercase();
    println!("Hi {} {}!", first_name, last_name);
}

