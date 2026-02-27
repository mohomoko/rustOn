///// make swap in Rust
/// This program demonstrates how to swap two integers in Rust using mutable references.
/// The `swap` function takes two mutable references to integers and swaps their values.
/// When you run this program, it will output the values of `x` and `y` before and after the swap.
/// To run this code, simply copy and paste it into a Rust project and execute it. You should see the output showing the values of `x` and `y` before and after the swap operation.
/// Output:
/// Before swap: x = 10, y = 15
/// After swap: x = 15, y = 10  

fn main() {
    let mut x = 10;
    let mut y = 15;

    println!("Before swap: x = {}, y = {}", x, y);

    swap(&mut x, &mut y);  // Pass mutable references to the swap function

    println!("After swap: x = {}, y = {}", x, y);
}

fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}