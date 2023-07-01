// Rust is a statically typed language,
// which means that it must know the types of all variables at compile time.
// The compiler can usually infer what type we want to use based on the value and how we use it. 

fn main() {

//! 1) Scalar Types
    //? A scalar type represents a single value. 
    //? Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    // 1.1) Integers
        //?  the u32 type. This type declaration indicates that the value it’s associated with
       //? should be an unsigned integer (signed integer types start with i instead of u)
      //? that takes up 32 bits of space.

    let guess: u32 = "42".parse().expect("Not a number!");

     // 1.2) Floating-Point Types
        //? Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size,respectively. 
        //? The default type is f64 because on modern CPUs, it’s roughly the same speed
        //? as f32 but is capable of more precision. All floating-point types are signed.
    
      
          let x = 2.0; // f64
      
          let y: f32 = 3.0; // f32

    // 1.3) Numeric Operations

      // addition
      let sum = 5 + 10;
  
      // subtraction
      let difference = 95.5 - 4.3;
  
      // multiplication
      let product = 4 * 30;
  
      // division
      let quotient = 56.7 / 32.2;
      let truncated = -5 / 3; // Results in -1
  
      // remainder
      let remainder = 43 % 5;

      // 1.3) The Boolean Type

        let t = true;
    
        let f: bool = false; // with explicit type annotation


      // 1.4) The Character Type

      let c = 'z';
      let z: char = 'ℤ'; // with explicit type annotation
      let heart_eyed_cat = '😻';

        //? Note that we specify char literals with single quotes, as opposed to string literals,
        //? which use double quotes. Rust’s char type is four bytes in size 
        //? and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
        //? Accented letters; Chinese, Japanese, and Korean characters; emoji;
        //? and zero-width spaces are all valid char values in Rust. 
        //?Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.


//! 2) Compound Types
    //? Compound types can group multiple values into one type.
    //? Rust has two primitive compound types: tuples and arrays.

        // 2.1) The Tuple Type
              //? A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
              //? Tuples have a fixed length: once declared, they cannot grow or shrink in size.
             //?We create a tuple by writing a comma-separated list of values inside parentheses. 
            //?Each position in the tuple has a type, and the types of the different values in the tuple 
           //?don’t have to be the same.

           let tup: (i32, f64, u8) = (500, 6.4, 1);

           //? The variable tup binds to the entire tuple because a tuple is considered a single compound element.
           //? To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.

           let (x, y, z) = tup;
          println!("The value of y is: {y}");

          //? We can also access a tuple element directly by using a period (.)
          //? followed by the index of the value we want to access.

          let five_hundred = tup.0;
          let six_point_four = tup.1;
          let one = tup.2;

          //? The tuple without any values has a special name, unit.
          //? This value and its corresponding type are both written () 
          //?and represent an empty value or an empty return type. 
          //?Expressions implicitly return the unit value if they don’t return any other value.

     //2.2) The Array Type
           //? Unlike a tuple, every element of an array must have the same type.
           //? Unlike arrays in some other languages, arrays in Rust have a fixed length.   

           let a = [1, 2, 3, 4, 5];

           //?  A vector is a similar collection type provided by the standard library
           //? that is allowed to grow or shrink in size. 

           let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

            //?Write an array’s type using square brackets with the type of each element,
            //? a semicolon, and then the number of elements in the array,

            let a: [i32; 5] = [1, 2, 3, 4, 5];

            //? You can also initialize an array to contain the same value for each element
            //? by specifying the initial value, followed by a semicolon,
            //? and then the length of the array in square brackets

            let a = [3; 5]; // let a = [3, 3, 3, 3, 3];

            // ACCESSING
            use std::io;

            let a = [1, 2, 3, 4, 5];
            println!("Please enter an array index.");
            let mut index = String::new();

            io::stdin()
           .read_line(&mut index)
           .expect("Failed to read line");

            let index: usize = index
           .trim()
           .parse()
           .expect("Index entered was not a number");

            let element = a[index];
            println!("The value of the element at index {index} is: {element}");

            //? The program resulted in a runtime error at the point of using an invalid value in the indexing operation.
            //? The program exited with an error message and didn’t execute the final println! statement.
            //? When you attempt to access an element using indexing, Rust will check 
            //? that the index you’ve specified is less than the array length.
            //? If the index is greater than or equal to the length, Rust will panic.
            //? This check has to happen at runtime, especially in this case,
            //? because the compiler can’t possibly know what value a user will enter when they run the code later.


}

