fn main() {
    // println!("name:");
    // let mut name: String = String::new();
    // std::io::stdin().read_line(&mut name);
    // println!("Hello, {name}");

    // println!("Numbers:");

    // let mut input: String = String::new();
    // std::io::stdin().read_line(&mut input);

    // let number: i32 = input
    //                         .trim()
    //                         .parse()  // convert to int
    //                         .expect("error: expected integer number");  // on ERROR
    
    // println!("{number}");

    //////////////////////////////

    // println!("list of numbers:");
    // let mut input_2: String = String::new();

    // std::io::stdin().read_line(&mut input_2);

    // let mut words = input_2.split(' ');

    // let first_word: &str = words.next().expect("error");

    // println!("{first_word}");

    // let second_word: &str = words.next().expect("error");

    // println!("{second_word}");

    //////////////////////////////////////
    

    println!("list of numbers (CSV):");
    let mut input: String = String::new();

    std::io::stdin().read_line(&mut input);

    let mut words = input.split(',');

    let first_number: &str = words.next().expect("error");

    println!("{first_number}");

    let second_number: &str = words.next().expect("error");

    println!("{second_number}");

}  
