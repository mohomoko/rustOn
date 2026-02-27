fn main() {

    /* tuple */

    let user = (
        String::from("Ali"),
        String::from("moho"),
        1990,
        80
    );

    println!("{} {}", user.0, user.1);

    let age = 2026 - user.2;
    println!("Age: {}", age);


    //////////////////////////////////
    /// 
    
    let point_2d = (3, 4);
    let (x, y) = point_2d;
    println!("x: {}, y: {}", x, y);

    ///////////////////////////////////
    
    let unit = ();  // we can consider unit() to be empty tuple (empty placeholder)


    /* nested tuple */

    let nested_tuple = (16, 2.5, (12, 13), String::from("test"));

    let value = nested_tuple.2 .0;  // access the first element of the inner tuple
    println!("{}", value);
    println!("{}", nested_tuple.2 .1);
}