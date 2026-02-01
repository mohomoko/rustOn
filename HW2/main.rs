fn main() {
    println!("Please input number:");

    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input);

    let number: i32 = input
                            .trim()
                            .parse()  // convert to int
                            .expect("error: expected integer number");  // on ERROR
    
    println!("{}", number * 2);

}
