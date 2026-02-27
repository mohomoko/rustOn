fn main() {
    let mut scores: [i32; 4] = [0; 4];  // just initialize with zeros

    scores[0] = 118;
    scores[1] = 110;
    scores[2] = 105;
    scores[3] = 100;
    
    let mut index = 0;
    
    // while index < scores.len() {
    //     println!("{}", scores[index]);
    //     index += 1;
    // }

    for element in scores {
        println!("{element}");
    }
    
    println!("=========================");

    // for value in scores {
    //     value = 12;  // error! value is just a temporary variable not the true memory of scores array!
    // }
    
}