struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//We used the owned String type rather than the &str string slice type. This is a deliberate 
// choice because we want each instance of this struct to own all of its data and for that data 
// to be valid for as long as the entire struct is valid.

// Rust allow us create a struct as tupple, 'tuple structs'
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;


fn main() {
    let user1 = User {
        active: true,
        username: String::from("BlockofChain"),
        email: String::from("blockoffchain@gmail.com"),
        sign_in_count: 1,
    };
    // to access one of the values of user1, use 'user1.username'
    // if struct of user1 is mutable, you can change any value of it with 'user1.xxxx = '

/* 
    // to create a user but don't change all the values wrt user1;
    // --snip--
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("blockofffchain@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
    //OR
    let user2 = User {
        email: String::from("blockofffchain@gmail.com"),
        ..user1
    };

    //In this example, we can no longer use user1 as a whole after creating user2 because 
    // the String in the username field of user1 was moved into user2. 
    // If we had given user2 new String values for both email and username, and thus only 
    // used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. 


*/

// Call the tuple structs
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// call the unit-like struct
let subject = AlwaysEqual;
//Imagine that later we’ll implement behavior for this type such that every instance of 
// AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. 




}

/*

fn build_use(username: String, email: String) -> User {
    User {
        active: True,
        username,
        email,
        sign_in_count: 1,
    }
}
 */

