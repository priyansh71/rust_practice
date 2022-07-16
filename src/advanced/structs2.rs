#[derive(Debug)] // a trait that adds the `Debug` implementation to the struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { 
    // methods
    fn area(&self) -> u32 { // taking a reference of the struct because we need to read
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // associated function, not a method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn fun(){

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Area : {}", rect.area());
    // println!("rect : {}", rect); // Display hasn't been implemented for Rectangle
    println!("Rect : {:#?}", rect); // Debug has been implemented for Rectangle
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    println!("Square : {:#?}", Rectangle::square(10));

}