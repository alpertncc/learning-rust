fn main() {
    /* Two data type subsets we have : 
            1-> Scalar
            2-> Compound
        Rust is a statically typed language, it must know the types of all variables.
    */

    let guess: u32 = "42".parse().expect("Not a number!"); /* Convert from String to Numaric Type */
    println!("{guess}");

    /* SCALAR TYPES 
            -> Represent a single value,
            -> 4 primary scalar types:
                --> Integers
                --> Floating-point numbers
                --> Booleans
                --> Characters
    */

        /* -> Integer Type 
        i -> signed
                u -> unsigned
                8/16/32/64/128 bits & arch ( isize / usize )    
        */

        /* -> Floating-Point Type
                -> f32 / f64 (32 bits / 64 bits)
        */ 

        /* -> Boolean Type
                -> true / false 
                -> 1 byte in size
        */ 

        /* -> Character Type
                -> char
                -> 'Z' single quotes
                -> four bytes in size
        */

    /* COMPOUND TYPES
            -> Can group multiple values into one type
            -> Tuples
            -> Arrays
        
        /* Tuple Type
                -> Have a fixed length 
                -> Once declared, cannot grow or shrink in size
                -> A tupple can have different types on it

                ->     let tup: (i32, f64, u8) = (500, 6.4, 1);
        
                ->     let tup = (500, 6.4, 1);
                       let (x, y, z) = tup;   // destructuring
                       println!("The value of y is: {y}");
                
                ->     let x: (i32, f64, u8) = (500, 6.4, 1);
                       let five_hundred = x.0;
                       let six_point_four = x.1;
                       let one = x.2;

                -> The tuple without any values has a special name, unit. 

        */

        /* Array Type
                -> Every element of an array must have the same type
                
                ->      let a = [1, 2, 3, 4, 5];
                
                ->      let a: [i32; 5] = [1, 2, 3, 4, 5];

                ->      let a = [3; 5]; ---> let a = [3, 3, 3, 3, 3];

                ->     let a = [1, 2, 3, 4, 5];
                       let second = a[1];
                       let first = a[0];
                



        */
*/

}
