fn main() {
    let mut scores: [i32; 5] = [0; 5];  // just initialize with zeros

    scores[0] = 12;
    scores[1] = 15;
    scores[2] = 20;
    scores[3] = 4;
    scores[4] = 18;
    
    let mut sum: i32 = 0;

    for element in scores {
        sum += element;
    }
    
    println!("{}", sum as f32 / 5.0);  // for float division, both should be float
}