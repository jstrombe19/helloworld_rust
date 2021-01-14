
/* multi-line comment */

// fn is keyword for function

// main is a mandatory function name, similar to C, C++, Java

// repl.it can be used as a web-based compiler to avoid having to compile on your computer each time
// play.rust-lang.org is also provided by the creators of Rust as a sandbox

fn main () {
  println!("Hello, World!");

  // print with placeholder
  println!("{}", "Hello, World!");

  // placeholder can also be used to print numeric values
  println!("{}", "2");

  // placeholders can also be used to print variables
  let a = "This is an arbitrary variable.";
  println!("{}", a);

  let be = false;
  println!("{}", be);
  println!("{}", !be);

  const MAX:i32 = 127;
  println!("MAX = {}", MAX);

  const MIN:i32 = MAX - 4;
  println!("MIN = {}", MIN);

  // this is a string from the Rust standard library
  let mut s = String::new();
  println!("This is the value of a new string s: {}", s);
  /* 
  If you are going to create a string in this way, where an empty string is instantiated above and defined later, 
  the only way to properly define its value is to use the String::from() method - if you attempt to strictly 
  define a new value, it will not compile because defining the value of a string directly is defining a string 
  slice while instantiating an empty string above is the string type from the standard Rust library
  */
  s = String::from("Hello");
  println!("This is the new value assigned to the string s: {}", s);

  // this is a string slice
  let str = "Hello";
  println!("{}", str);

  // below illustrates simple operators in action as well as passing multiple values to a println! function
  let mut c = 10;
  println!("The initial value of c is {}. In the next line, it will be incremented by a previously specified value 'MAX', whose value is {}.", c, MAX);
  c += MAX;
  println!("The new value of c is {}", c);

  // shadowing

  let d:f32 = 1.32;
  println!("The value of d1 - an f32 variable - is {}", d);
  let d:i8 = 4;
  println!("The value of d2 - an i8 variable - is {}", d);
  let d:char = 'c';
  println!("The value of d3 - a char variable - is {}", d);
  let d:f64 = 1.32;
  println!("The value of d4 - and f64 variable - is {}", d);
}
