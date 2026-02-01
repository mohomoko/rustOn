fn main() {
    let b_true: bool = true;
    let b_false: bool = false;

    let mut score = 15;
    let mut result : bool = score > 10;

    println!("line 8: result is {}", result);  // answer: true

    score = 10;
    result = score >= 10;
    println!("line 12: result is {}", result);  // answer: true

    score = 20;
    result = score <= 20;
    println!("line 16: result is {}", result);  // answer: true

    score = 20;
    result = score == 20;
    println!("line 20: result is {}", result);  // answer: true
    
    result = !b_true;
    println!("line 23: result is {}", result);  // answer: false

    score = 7;
    result = !(score >= 10);
    println!("line 27: result is {}", result); // answer: true

    let inp_1 = String::from("Alice");
    let inp_2 = String::from("Alice");

    result = inp_1 == inp_2;
    println!("line 32: result is {}", result); // answer: true

    result = !(inp_1 == inp_2);
    println!("line 35: result is {}", result); // answer: false

    result = inp_1 != inp_2;
    println!("line 38: result is {}", result); // answer: false

}
