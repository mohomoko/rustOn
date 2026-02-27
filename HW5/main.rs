// solution for HW5; Ceremony
fn main() {
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    let mut xy = input.split(' ');

    let x_str: &str = xy.next().expect("error");
    let x: u8 = x_str.trim().parse().expect("first input is not uint!");
    let y_str: &str = xy.next().expect("error");
    let y: u8 = y_str.trim().parse().expect("second input is not uint!");

    let right_or_left: String = if (0 < x) & (x <= 10) {
        String::from("Right")
    } else {
        String::from("Left")
    };
    let row = 11 - y;
    let col: u8 = if right_or_left == String::from("Left") {
        21 - x
    } else {
        x
    };
    
    println!("{} {} {}", right_or_left, row, col);
}