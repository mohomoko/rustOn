fn main() {
    let username = String::from("moho");
    let password = String::from("12@qwe");

    'username_loop: loop{
        println!("==== Login ====");
        println!();
        println!("username: ");  // How can i grab input inline with this? (HELP: must use flush...)
        
        let mut input_username = String::new();
        std::io::stdin().read_line(&mut input_username);
        
        // if input_username == username {  // It is not working! because input contains "\n"! we can use that with `.trim()` function!
        if input_username.trim() != username {  // It is OK 
            println!("Wrong Username");
            continue;
            
        }
        

        loop{  // for grabbing PASSWORD ONLY, not username again!
            println!("password: ");
            let mut input_password = String::new();
            std::io::stdin().read_line(&mut input_password);
            if input_password.trim() != password {
                println!("Wrong Password");
                continue;
            }
            break 'username_loop;  // naming loop to brake username_loop and go to "login"!
        }
    }

    println!("login")

    // let result = 'first_loop : loop{
    //     println!("loop 1");
    //     loop {
    //         println!("loop 2");
    //         loop {
    //             println!("loop 3");
    //             loop{
    //                 println!("loop 4");
    //                 break 'first_loop 6;  // Works on nearest loop ( ln 4 )
    //             }
    //         }
    //     }
    // };

    // println!("{result}")
    
    /*  
     * output:
     * loop 1 
     * loop 2
     * loop 3
     * loop 4
     * 6
     */



}