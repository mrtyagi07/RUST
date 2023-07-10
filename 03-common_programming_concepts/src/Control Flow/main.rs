//! if Expressions

fn main() {
  let number = 3;

  if number < 5 {
      println!("condition was true");
  } else {
      println!("condition was false");
  }
}








// It’s also worth noting that the condition in this code must be a bool. 
// If the condition isn’t a bool, we’ll get an error. For example, try running the following code

fn main() {
  let number = 3;

  if number {                 // ^^^^^^ expected `bool`, found integer
      println!("number was three");
  }
}

// The error indicates that Rust expected a bool but got an integer. 
// Unlike languages such as Ruby and JavaScript, 
// Rust will not automatically try to convert non-Boolean types to a Boolean. 
// You must be explicit and always provide if with a Boolean as its condition.

//?  If we want the if code block to run only when a number is not equal to 0,
//? for example, we can change the if expression to the following:

fn main() {
  let number = 3;

  if number != 0 {
      println!("number was something other than zero");
  }
}

//! Handling Multiple Conditions with else if

fn main() {
  let number = 6;

  if number % 4 == 0 {
      println!("number is divisible by 4");
  } else if number % 3 == 0 {
      println!("number is divisible by 3");
  } else if number % 2 == 0 {
      println!("number is divisible by 2");
  } else {
      println!("number is not divisible by 4, 3, or 2");
  }
}


//! Using if in a let Statement
    // Because if is an expression, we can use it on the right side of a let statement
    // to assign the outcome to a variable

    fn main() {
      let condition = true;
      let number = if condition { 5 } else { 6 };
  
      println!("The value of number is: {number}");
  }

  //? if expression depends on which block of code executes. 
  //?This means the values that have the potential to be results from each arm of the
  //? if must be the same type;  the results of both the if arm and the else arm were i32 integers.
  //? If the types are mismatched, as in the following example, we’ll get an error:

  fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" }; 

    println!("The value of number is: {number}");
}

// This won’t work because variables must have a single type, and Rust needs to know at compile time
// what type the number variable is, definitively. Knowing the type of number lets the compiler verify
// the type is valid everywhere we use number. Rust wouldn’t be able to do that if the type of number
// was only determined at runtime; the compiler would be more complex and would make fewer guarantees
// about the code if it had to keep track of multiple hypothetical types for any variable.

//! Repetition with Loops
    //*Repeating Code with loop */

    //? The loop keyword tells Rust to execute a block of code over and over again forever
    //? or until you explicitly tell it to stop.

    fn main() {
      loop {
          println!("again!");
      }
  }

  //?When we run this program, we’ll see again! printed over and over continuously until we stop the program manually.
  //? You can place the break keyword within the loop to tell the program when to stop executing the loop

  //*Returning Values from Loops */

  //? One of the uses of a loop is to retry an operation you know might fail,
  //? such as checking whether a thread has completed its job. 
  //? You might also need to pass the result of that operation out of the loop to the rest of your code. 
  //? To do this, you can add the value you want returned after the break expression 
  //? you use to stop the loop; that value will be returned out of the loop so you can use it

  fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

//* Loop Labels to Disambiguate Between Multiple Loops */

//? If you have loops within loops, break and continue apply to the innermost loop at that point. 
//? You can optionally specify a loop label on a loop that you can then use with break or continue 
//? to specify that those keywords apply to the labeled loop instead of the innermost loop. 
//? Loop labels must begin with a single quote.

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

//* Conditional Loops with while */

fn main() {
  let mut number = 3;

  while number != 0 {
      println!("{number}!");

      number -= 1;
  }

  println!("LIFTOFF!!!");
}

//* Looping Through a Collection with for */
fn main() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
      println!("the value is: {}", a[index]);

      index += 1;
  }
}

//? However, this approach is error prone; we could cause the program to panic
//? if the index value or test condition is incorrect. For example, 
//? if you changed the definition of the a array to have four elements but forgot to update the condition
//? to while index < 4, the code would panic. It’s also slow, 
//? because the compiler adds runtime code to perform the conditional check of whether 
//? the index is within the bounds of the array on every iteration through the loop.

fn main() {
  let a = [10, 20, 30, 40, 50];

  for element in a {
      println!("the value is: {element}");
  }
}

// The safety and conciseness of for loops make them the most commonly used loop construct in Rust. 
// Even in situations in which you want to run some code a certain number of times, 
// as in the countdown example that used a while loop, most Rustaceans would use a for loop. 
// The way to do that would be to use a Range, provided by the standard library, 
// which generates all numbers in sequence starting from one number and ending before another number.

//? Here’s what the countdown would look like using a for loop and
//? another method we’ve not yet talked about, rev, to reverse the range:

fn main() {
  for number in (1..4).rev() {
      println!("{number}!");
  }
  println!("LIFTOFF!!!");
}