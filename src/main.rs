fn main() {
    let rectangle: (u32, u32) = (30, 50);
    println!(
        "the area of this rectangle is {} square pixels.",
        area(rectangle)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}