fn main() {
    // println!("Start");

    // loop {
    //     println!("again");
    // }

    // println!("End");  // We NEVER Reach here....

    // ////////////////////////
    
    // println!("Start");
    
    // // while true {  // Equal to loop command (Infinite-loop!)
    // //     println!("again")
    // // }

    // while false {  // never executed!
    //     println!("again")
    // }

    // println!("End")

    // /////////////////////////
    
    // println!("Start");
    
    // let mut counter = 0;
    // while counter < 5 {
    //     println!("*");
    //     counter += 1;  // counter = counter + 1
    // }
    
    // println!("End");

    // /*
    //  *    output:
    //  *    **********
    //  *    **********
    //  *    **********
    //  *    **********
    //  *    **********
    //  * 
    //  */

    //////////////////////////////
    // nested loop //
    let mut first_counter = 0;
    
    while first_counter < 5 {
        let mut secound_counter=0;
        
        while secound_counter<10 {
            print!("*");
            secound_counter += 1
        }

        println!();
        first_counter += 1;  // counter = counter + 1
    }

    /*
     *    output:
     *    **********
     *    **********
     *    **********
     *    **********
     *    **********
     * 
     */

}