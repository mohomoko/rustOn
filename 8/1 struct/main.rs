struct Book {
    name: String,
    price: f64,
    pages: u32,
    in_stock: bool,
}

fn main() {
    // // Define a struct could be inside of the function too, but it's more common to define it outside
    // struct Book {
    //     name: String,
    //     price: f64,
    //     pages: u32,
    //     in_stock: bool,
    // }

    let b1 = Book {
        name: String::from("The Old Man and the Sea"),
        price: 13.99,
        pages: 128,
        in_stock: true,
    };  // b1 is Instance of Book

    let price = b1.price;
    println!("{} costs ${}", b1.name, price);

    let mut b2 = Book {
        name: String::from("The Stranger"),
        price: 15.0,
        pages: 123,
        in_stock: true,
    };

    b2.price = 9.99 ;  // Update the price of b2

    // consider ownership and borrowing
    let name = b2.name;
    // println!("{}", b2.name);  // This will cause a compile-time error because b2.name has been moved to name
    println!("{}", b2.price);  // This is still valid because just name was moved, not the entire b2 struct

}