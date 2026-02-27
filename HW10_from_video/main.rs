// Sort x , y: if y < x, swap them

fn main() {
    let mut x = 10;
    let mut y = 15;

    println!("Before swap: x = {}, y = {}", x, y);

    swap_if_y_lt_x(&mut x, &mut y);  // Pass mutable references to the swap function

    println!("After swap: x = {}, y = {}", x, y);
}

fn swap_if_y_lt_x(x: &mut i32, y: &mut i32) {
    if *y < *x {
        let temp = *x;
        *x = *y;
        *y = temp;
    }
}