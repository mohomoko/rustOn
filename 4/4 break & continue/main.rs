fn main() {
    // println!("Start");

    // loop {
    //     println!("loop begin");
    //     break;
    //     println!("loop end")  // We NEVER Reach here....
    // }

    // println!("End");

    // ////////////////////////

    // let mut inputNumber: i32 ;

    // loop {
    //     println!("Please choose an Option:");
    //     println!();
    //     println!("1) Start");
    //     println!("2) About");
    //     println!("3) Exit");
    //     println!();
    //     println!(">>");

    //     let mut inputString: String = String::new();
    //     std::io::stdin().read_line(&mut inputString);
    //     inputNumber = inputString.trim().parse().expect("error: input Must be an integer");

    //     if inputNumber == 1 || inputNumber == 2 || inputNumber == 3 {
    //         break;
    //     } else {
    //         println!("Invalid Input!");
    //         println!();
    //     }
    // }
    // println!("{inputNumber}");
    
    // ///////////////////////////////
    
    // println!("Start");

    // loop {
    //     println!("loop begin");
    //     continue;
    //     println!("loop end")  // We NEVER Reach here....
    // }

    // println!("End");  // AND NEVER we Reach here too....

    // /////////////////////////
    
    // finding 
    let mut number =0;
    while number < 100 {
        if number % 12 != 0 {
            number += 1;
            continue;
        }

        println!("{}", number);
        number += 1;
    }

    /*
     *    output:
     *    0
     *    12
     *    24
     *    36
     *    48
     *    60
     *    72
     *    84
     *    96
     */

}