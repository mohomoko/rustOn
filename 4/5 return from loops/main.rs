fn main() {
    // let result = loop{

    // // error: Could not do that with WHILE
    // // let result = while {

    //     println!("begin");
    //     break 6;  // can return value but ...
    //     println!("end");
    // };  // need semicolumn here!

    // println!("{result}");

    // //////////////////////////////////

    let mut number = 5000;
    let result = loop{
        if number % 12 == 0 {
            break number;
        }
        number -= 1;
    };
    println!("{result}");

}