// fn main() {
//     let r = add(3, 4);
//     print!("{r}");
// }

// // fn add(x: i32, y: i32) -> i32 {
// //     let result = x + y;
// //     return result;
// // }

// // fn add(x: i32, y: i32) -> i32 {
// //     let result = x + y;
// //     result  // NO SEMICOLON means this is the return value of the function
// // }

// // fn add(x: i32, y: i32) -> i32 {
// //     return x + y;
// // }

// fn add(x: i32, y: i32) -> i32 {
//     x + y  // NO SEMICOLON -> return value
// }

/////////////////////////////////////

// fn main() {
//     do_something();
// }

// fn do_something() {
//     println!("begin");
//     return;
//     println!("end");  // this will never be executed
// }

fn main() {
    let (addition, subtraction, multiplication, division) = calculate_basic_operations(10, 5);
    println!("{addition}");
    println!("{subtraction}");
    println!("{multiplication}");
    println!("{division}");
}

fn calculate_basic_operations(x: i32, y: i32) -> (i32, i32, i32, i32) {
    (x + y, x - y, x * y, x / y)
}