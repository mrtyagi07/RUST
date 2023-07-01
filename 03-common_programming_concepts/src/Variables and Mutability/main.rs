//Variables and Mutability


fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constants aren’t just immutable by default—they’re always immutable.
    // You declare constants using the const keyword instead of the let keyword,
    // and the type of the value must be annotated

    //Constants can be declared in any scope, including the global scope,
    // which makes them useful for values that many parts of code need to know about.
   //The last difference is that constants may be set only to a constant expression,
  // not the result of a value that could only be computed at runtime.

  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

  //Shadowing


    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

 //Shadowing is different from marking a variable as mut because we’ll get a compile-time error
 // if we accidentally try to reassign to this variable without using the let keyword.
 // By using let, we can perform a few transformations on a value but have the variable be immutable
 // after those transformations have been completed.

 //!The other difference between mut and shadowing is that 
 //! because we’re effectively creating a new variable when we use the let keyword again,
 //!  we can change the type of the value but reuse the same name.
    let spaces = "   "; //string
    let spaces = spaces.len(); //number

}
