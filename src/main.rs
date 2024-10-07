#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of this rectangle is {} square pixels.",
        area(&rect1)
    );

    //let's have some fun!
    // println!("rect1 is {}", rect1); //! Rectangle` doesn't implement `std::fmt::Display
    //? structs don’t have a provided implementation of Display to use with println! and the {} placeholder.
    //Trying again...
    //println!("rect1 is {:?}", rect1); //! error[E0277]: `Rectangle` doesn't implement `Debug`
    //? add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`

    //* Last try...
    println!("rect1 is {:#?}", rect1); //* rect1 is Rectangle { width: 30, height: 50 }
    

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
