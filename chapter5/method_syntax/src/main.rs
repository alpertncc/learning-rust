#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
/*
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area is {}", rect.area());
}
 */

 /*
 impl Rectangle {   
    fn width(&self) -> bool {
        self.width > 0
    }
 }

 fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The nonzero width is {}", rect.width);
 }
 */


 /* 
 // Methods with More Parameters

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}
*/

/* 
// Associated Funtions

    // All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
 // To call this associated function, we use the :: syntax with the struct name
fn main() {
    let sq = Rectangle::square(3);
    println!("{:#?}",sq);
}
*/


// Multiple impl blocks

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}





















