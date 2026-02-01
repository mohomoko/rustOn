fn main() {
    let b_true: bool = true;
    let b_false: bool = false;

    // OR
    println!("true | true = {}", b_true | b_true);  // answer: true
    println!("true | false = {}", b_true | b_false); // answer: true
    println!("false | true = {}", b_false | b_true); // answer: true
    println!("false | false = {}", b_false | b_false); // answer: false

    println!("===================================");

    // AND;
    println!("true & true = {}", b_true & b_true);  // answer: true
    println!("true & false = {}", b_true & b_false); // answer: false
    println!("false & true = {}", b_false & b_true); // answer: false
    println!("false & false = {}", b_false & b_false); // answer: false

    println!("===================================");

    // XOR
    println!("true ^ true = {}", b_true ^ b_true);  // answer: false
    println!("true ^ false = {}", b_true ^ b_false); // answer: true
    println!("false ^ true = {}", b_false ^ b_true); // answer: true
    println!("false ^ false = {}", b_false ^ b_false); // answer: false

}
