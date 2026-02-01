// solution for Yakhdarchi HW3

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input);

    let number: i32 = input
                            .trim()
                            .parse()  // convert to int
                            .expect("error: expected integer number");  // on ERROR

    if number > 100 {
        println!("Steam");
    } else if number >= 0 {
        println!("Water");
    } else {
        println!("Ice");
    }
}