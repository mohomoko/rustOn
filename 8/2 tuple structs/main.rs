struct Book{
    name: String,
    price: f64,
    pages: i32,
    in_stock: bool,
}

fn main() {
    // tuple
    let rgb_color_red = (255, 0, 0);

    // tuple struct
    struct RGB(i32, i32, i32);
    let red = RGB(255, 0, 0);

    println!("{}", rgb_color_red.0);
    println!("{}", red.0);
    
    // Unit like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // Instances from other Instances
    let b1 = Book{
        name: String::from("The Old Man and the Sea"),
        price: 13.99,
        pages: 128,
        in_stock: true,
    };

    let b2 = Book {price: 8.0, ..b1 };

    println!("{}", b2.name);
    // println!("{}", b1.name);  // b1 is still valid because we only changed the price field, the rest of the fields are moved from b1 to b2
    println!("{}", b1.price);  // no error


}