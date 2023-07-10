//!Convert temperatures between Fahrenheit and Celsius.

// Function to convert temperature from Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
  let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
  celsius
}

// Function to convert temperature from Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
  let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
  fahrenheit
}

fn main() {
  let fahrenheit: f32 = 98.6;
  let celsius = fahrenheit_to_celsius(fahrenheit);
  println!("{} degrees Fahrenheit is equal to {} degrees Celsius.", fahrenheit, celsius);

  let celsius: f32 = 37.0;
  let fahrenheit = celsius_to_fahrenheit(celsius);
  println!("{} degrees Celsius is equal to {} degrees Fahrenheit.", celsius, fahrenheit);
}

fn fibonacci_number(n: i32) -> i32 {
  if n <= 0 {
      return 0;
  } else if n == 1 {
      return 1;
  }

  let mut fib_n_minus_2 = 0;
  let mut fib_n_minus_1 = 1;
  let mut fib_n = 0;

  for _ in 2..=n {
      fib_n = fib_n_minus_2 + fib_n_minus_1;
      fib_n_minus_2 = fib_n_minus_1;
      fib_n_minus_1 = fib_n;
  }

  fib_n
}

fn main() {
  let n = 5;
  let fib_n = fibonacci_number(n);
  println!("The {}th Fibonacci number is: {}", n, fib_n);
}


