fn main () {
   /*
    let x = 5; 
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    /* X is immutable variable, hence you will take an error. */
    */


    /*
    let mut x = 5;  /* mut will make your variable mutable. */
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}"); 
    */

    /*  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  */


    /*  SHADOWING
    let x = 5;  (shadowed by x = x + 1)

    let x = x + 1;

    {
        let x = x * 2;      (inner shadow, it ends when the scope is over)
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    */

    /*
    let _spaces = "   ";
    let _spaces = _spaces.len();
    */

    /*          (due to "mut", we cannot change the type of _spaces)
    let mut _spaces = "   ";
    _spaces = _spaces.len();
    */

}