struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {

    /*
     * All functions defined within the impl block are called associated functions, because they’re associated with the type named after impl.
     */

    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.width * rectangle.height
    // }  // This is Associated Function, because it’s associated with the type named after impl, but it’s not a method because it doesn’t have self as its first parameter

    fn area(&self) -> u32 {  /* `& mut self` or `self`, is my choise */
        self.width * self.height
    }  // This is Method, because it’s associated with the type named after impl and it has self as its first parameter

    fn width(&self) -> bool {
        self.width > 0
    }

    fn update_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

impl Rectangle {
    fn do_something() -> String {
        String::from("Doing something...")
    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(rectangle: Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     Rectangle::area(&rect1)
    // );

    /**** diffrence of Methods and Functions ****/

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // This is Method, Unlike functions, methods are defined within the context of a struct and their first parameter is always self
    );
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect2_area = rect2.area();
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2_area
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let mut rect3 = Rectangle {
        width: 30,
        height: 20,
    };

    rect3.update_size(60, 45);

    println!("rect3's new width is {} and height is {}", rect3.width, rect3.height);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let s = Rectangle::do_something();  // This is Associated Function

    println!("{}", s);

}