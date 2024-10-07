#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40
    };

    let rect3 = Rectangle {
        width: 30,
        height: 70
    };

    println!("rect1 can hold rect2 => {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 => {}", rect1.can_hold(&rect3));

    println!(
        "The area of this rectangle is {} square pixels.",
        rect1.area()
    );

    //let's have some fun!
    // println!("rect1 is {}", rect1); //! Rectangle` doesn't implement `std::fmt::Display
    //? structs don’t have a provided implementation of Display to use with println! and the {} placeholder.
    //Trying again...
    //println!("rect1 is {:?}", rect1); //! error[E0277]: `Rectangle` doesn't implement `Debug`
    //? add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`

    //* Last try...
    // println!("rect1 is {:#?}", rect1); //* rect1 is Rectangle { width: 30, height: 50 }
}
