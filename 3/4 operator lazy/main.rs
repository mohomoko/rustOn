use std::result;

fn main() {
    let b_true: bool = true;
    let b_false: bool = false;

    // OR
    println!("true || true = {}", b_true || b_true);  // answer: true
    println!("true && true = {}", b_true && b_true);  // answer: true
    
    // println!("true ^ true = {}", b_true ^^ b_true);  // answer: error
    
    // panic();  // error: uncommenting this line will cause the program to panic immediately

    // let result = b_true | panic!();  // error: panic() is evaluated
    let result = b_true || panic!();  // ok: short-circuited and panic() is not evaluated
    println!("result is {}", result); 

    let result = b_false && panic!();  // ok: short-circuited and panic() is not evaluated
    println!("result is {}", result);

    /*
     *
     * Summary:
     *  - Logical OR (||) and AND (&&) operators short-circuit evaluation.
     *  - Bitwise OR (|) and AND (&) operators do not short-circuit evaluation.
     *  - There is no logical XOR operator in Rust; use bitwise XOR (^) instead.
     *  - we Usually use logical operators (||, &&) for boolean logic to increase performance.
     *  - But when we need to run/check all conditions, we Must use bitwise operators (|, &).
     */
    
}
