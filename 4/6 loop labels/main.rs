fn main() {
    // loop{
    //     println!("loop 1 ");
    //     loop{
    //         println!("loop 2");
    //         break;  // Works on nearest loop ( ln 4 )
    //     }
    // }
    
    // /*  
    //  * output:
    //  * loop 1 
    //  * loop 2
    //  * loop 1 
    //  * loop 2
    //  * ...
    //  * ..
    //  * .
    //  */

    // /////////////////////
    
    // 'first_loop : loop{
    //     println!("loop 1 ");
    //     loop{
    //         println!("loop 2");
    //         break 'first_loop;  // Works on nearest loop ( ln 4 )
    //     }
    // }

    // /*  
    //  * output:
    //  * loop 1 
    //  * loop 2  // and the code will reach end
    //  */

    // //////////////////////////////
    
    // 'first_loop : loop{
    //     println!("loop 1");
    //     loop {
    //         println!("loop 2");
    //         'third_loop:loop {
    //             println!("loop 3");
    //             loop{
    //                 println!("loop 4");
    //                 break 'third_loop;  // Works on nearest loop ( ln 4 )
    //             }
    //         }
    //     }
    // }
    
    // /*  
    //  * output:
    //  * loop 1 
    //  * loop 2
    //  * loop 3
    //  * loop 4
    //  * loop 2
    //  * loop 3
    //  * loop 4
    //  * ...
    //  * ..
    //  * .
    //  */

    //////////////////////
    
    let result = 'first_loop : loop{
        println!("loop 1");
        loop {
            println!("loop 2");
            loop {
                println!("loop 3");
                loop{
                    println!("loop 4");
                    break 'first_loop 6;  // Works on nearest loop ( ln 4 )
                }
            }
        }
    };

    println!("{result}")
    
    /*  
     * output:
     * loop 1 
     * loop 2
     * loop 3
     * loop 4
     * 6
     */


    // error: Could not do th at with WHILE
    // let result = while {

    
}