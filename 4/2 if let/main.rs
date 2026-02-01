fn main() {
    // let authentication = String::from("OK");

    // let result: bool = if authentication == "OK" {true} else {false};  // Using if-else expression to assign boolean value

    // println!("Authentication result: {}", result);


    // //////////////
    
    let number:i32 =11;

    let mut result: String = String::new();

    if number % 2 == 0 {
        result = String::from("number is divisible by 2");
    } else {
        result = String::from("number is not divisible by 2");
    }

    println!("{result}");

    // is equal to:
    let mut result: String = if number % 2 == 0 {
        String::from("number is divisible by 2")
    } else {
        String::from("number is not divisible by 2")
    };

    println!("{result}");

}