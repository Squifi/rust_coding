// Need the annotation in order to print out Debug information
#[derive(Debug)]
struct Rectangle  {
    width: u32,
    height: u32,
}

fn main() {
    // single vars for each 
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_weird(width1, height1)
        );
 
    // Tuple example
    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
        );

    // Nice struct
    let rect2 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect2)
        );

    // Will not print as structs do not have Display by default.
    // Hence the specifier: '{:?}'
    println!("rect2 is {:?}", rect2);
    
    // Using the specifier {:#?} makes the output easier to read.
    println!("\nrect2 is {:#?}", rect2);
}

fn area_weird(width: u32, height: u32) -> u32 {
    width * height
} 

fn area_tuple(dimension: (u32, u32)) -> u32 {
    // weird with .0 and .1 magic numbers
    dimension.0 * dimension.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
