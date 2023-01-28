// The purpose of this example is to calculate the area of a rectangle.


/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels. ", area(width1,height1));

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
 */


/*
 fn main() {
    let rect = (30,50);

    println!("The area is {}", area(rect));
 }

 fn area(dimension: (u32,u32)) -> u32 {
    dimension.0 * dimension.1
 }

 */

/*

// More understandable form

 struct Rectangle {
    width: u32,
    height: u32,
 }

 fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area is {}", area(&rect));
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
 */



/* 
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

//    println!("The rectangle is {}", rect); // This code will cause an error.
//    println!("The rectangle is {:?}", rect);
//    println!("The rectangle is {:#?}", rect);
}
*/

/* 
 #[derive(Debug)]
 struct Rectangle {
    width: u32,
    height: u32,
 }
 
 fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect);
 }
 
 */
