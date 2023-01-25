/* 

fn main() {

    let condition = true ;
    let x = if condition { 5 } else { 6 };
    println!("{x}");
}
*/ 

/* -> LOOPS
        --> loop , while , for
*/ 
    /* -> Loop
            --> Executes a block of code over and over again forever or until we explicitly tell it to stop
            
            --> 
                fn main() {
                  loop {
                  println!("again!");
                  }
                }

        */

/*
fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");

}
*/

/*
fn main() {

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } 
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;

    }
    println!("End count = {count}");

}
*/

/*
fn main() {

    let mut counter = 3;
    while counter != 0 {
        println!("{counter}");
        counter -= 1;
    }
    println!("LISTOFFF!!!!!")


}
*/

/*
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
*/

/*
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
*/

/*
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
*/



































