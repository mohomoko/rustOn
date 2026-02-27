fn main() {
    // let my_array = [12, 15, 9];

    // let first_element = my_array[0];

    // println!("{first_element}");

    // println!("{}", my_array[0]);
    // println!("{}", my_array[1]);
    // println!("{}", my_array[2]);

    // let cities = ["Tehran", "Shiraz"];

    // println!("{}", cities[0]);

    // let number: i32 = 12;
    // let scores: [i32; 5] = [118, 110, 105, 100, 102];

    // let numbers = [3; 5];

    // println!("{}", numbers[0]);
    // println!("{}", numbers[1]);
    // println!("{}", numbers[2]);
    // println!("{}", numbers[3]);
    // println!("{}", numbers[4]);
    // // println!("{}", numbers[5]);  // 26 |     println!("{}", numbers[5]); ->  | index out of bounds: the length is 5 but the index is 5


    ////////////////////////////////////////////////
    
    // let scores: [i32; 5] = [118, 110, 105, 100, 102];
    let mut scores: [i32; 5] = [0; 5];  // just initialize with zeros

    scores[0] = 118;
    scores[1] = 110;
    scores[2] = 105;
    scores[3] = 100;
    scores[4] = 102;
    
    // scores[14] = 116;
    
    let mut index = 0;
    
    // while index < 5 {
        //     println!("{}", scores[index]);
        //     index += 1
        // }
        
        while index < scores.len() {
            println!("{}", scores[index]);
            index += 1
        }
        
    println!("=======================");
    
    // println!("{}", scores);  // reach error
    println!("{:?}", scores);
    
    println!("=======================");
    
    println!("{:#?}", scores);
    
    println!("=======================");
    
    let mut index = 0;
    while index < scores.len() {
        println!("scores {} => {}", index+1,  scores[index]);
        index += 1;
    }
    
    println!("=======================");
    
    let mut index = 0;
    
    while index < scores.len() {
        scores[index] = 12;
        index += 1;
    }
    
    let mut index = 0;
    while index < scores.len() {
        println!("scores {} => {}", index+1,  scores[index]);
        index += 1;
    }

}