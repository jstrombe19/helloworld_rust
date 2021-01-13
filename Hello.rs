
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
}
