#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block for the Rectangle struct
// Helps keep things organised, so future users will not
// have to search for an area function, but know where to 
// find the methods.
impl Rectangle {
    // The method borrows self and main keeps the ownership
    // We do not want to write to the Object, just read the data.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width &&
            self.height > other_rect.height
    }

    // Signature of the method does not have to contain self
    // &self or &mut self. If the self param is missing these methods are called
    // Associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!(
        "\nThe area of the rectangle is {} square pixels.\n",
        // Rust has automatic referencing and dereferencing here
        // no need for &, & mut or * here. puh :)
        rect.area()
        );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // use Rectangle::associated_function to access associated functions.
    println!("\nCreated a nice square rectangle {:#?}", Rectangle::square(40));

}
