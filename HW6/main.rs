// solution for HW6; Wow!
fn main() {
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    let mut number : u8 = input.trim().parse().expect("Type uint number");

    print!("W");
    while number > 0 {
        print!("o");
        number -= 1
    }
    println!("w!");
}