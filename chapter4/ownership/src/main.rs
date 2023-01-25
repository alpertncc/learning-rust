/*
fn main() {
    // let s = "hello";
    // let s = String::from("hello");
    /*
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); 
    */

    /*
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world");  // s1 removed
    */

    /*
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 : {s1}, s2 : {s2}");
    */

    /* Some of types that implement "Copy";
    -> All the integer types,
    -> Boolean type
    -> All the floating-point types
    -> The character type,
    -> Tuples (only with the types which can be implement "Copy")
    */

}
*/



/*
// Ownership & Functions

fn main() {

    let s = String::from("hello");

    takes_ownership(s);  // 's' is no longer valid for main function
                         // 's's is moved to takes_ownership function
    let x = 5;  

    makes_copy(x);       // 'x' is valid on both main and makes_copy functions
                         // 'x' can be used in the main function

}   // Here, x goes out of scope, then s. THE STACK

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}   // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}   // Here, some_integer goes out of scope. Nothing special happens. 
*/


/*
// Return Values and Scope

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
} //  Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {  // gives_ownership will move its return value into the function that calls it.
    let some_string = String::from("yours");
    some_string // some string is returned and moves out to the calling function 
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { 
    a_string 
}
*/

/*
fn main(){
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is '{len}'")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
*/

/*
// References & Borrowing
// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

fn main(){
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is '{len}'")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
*/

/*
// Mutable References
fn main() {
    let mut s1 = String::from("Hello");
    change(&mut s1);
    println!("{s1}");
}

fn change(some_string: &mut String) {
    some_string.push_str(" World");
} 

// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

// As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
fn main() {
  let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
// We also cannot have a mutable reference while we have an immutable one to the same value.
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}


fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

*/