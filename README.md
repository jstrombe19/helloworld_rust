all values in Rust are of a specific data type; Rust is statically typed

Rust can and does infer data types when using single data types; however, anything that involves
multiple data types requires those data types to be explicitly defined

two classes of data types: scalar and compound

there are four primary scalar data types: integers, floating-point numbers, booleans, and characters

| Bits | Signed | Unsigned |
| :--- | :----: | -------: |
| 8    | i8     | u8       |
| 16   | i16    | u16      |
| 32   | i32    | u32      |
| 64   | i64    | u64      |
| 128  | i128   | u128     |
| arch | isize  | usize    |

* the 'arch' defers size to that of the architecture on which the program compiles - if we have a 32-bit system architecture, then
the signed and unsigned values will be 32 bits

* the limit for integer values is equal to 2^(n-1) - 1 for positive values and 2^(n-1) for negative values
** IF a value greater than the maximum is stored, integer overflow occurs and the value is converted to its corresponding position in the range
** for example, i8 can only store a maximum of 255. if 256 is entered, the output will be 0. 257 will be 1, 258 will be 2, and so on...

Rust has 2 primitive types for floating-point numbers: f32 and f64

* Rust will not automatically cast an integer value as a float - even if the value is a whole number, it must be entered with the .0 included to avoid compilation failure

** VARIABLES IN RUST, BY DEFAULT, ARE IMMUTABLE **

* in order to make variables mutable, the keyword 'mut' must be appended to its definition as follows:
let mut variable = value

variably constant

variables defined with 'let' will be immutable unless the 'mut' keyword is included in their declaration; however, there are still constants in Rust

* constants are absolutely fixed variables - they cannot be changed at any point after their assignment

* constants' data types must also be explicitly defined

STRINGS

- Rust has only one string type in its *core* language - the string slice (&str)
* The Rust standard library provides an additional string type (string), which provides additional behavior such as mutability
* string slices are immutable

OPERATORS

Rust supports all but two standard operators. +, -, /, *, %, <, > are all supported; ++ and -- are not.

SHADOWING

Rust allows users to define multiple variables with the same name in the same scope, with the same or different assigned values. This is not the same as making a variable mutable. Mutable variables are variables whose assigned values may be changed throughout the course of a program. The value is changed, but it is a single variable throughout. Shadowing is declaring multiple variables of the same name but with different values. Because each variable is unique, each can have a different value - not only within the same data type but even values of different data types. Order of operations is paramount with shadowing, as only the most recent variable is accessible at any point in the execution of the code. If I have a variable 'd', d1 (the first instance of d) is available throughout the program at any point after its declaration and definition until the second instance of 'd' (d2) is declared. After d2 is declared and defined, only d2 is available - access to d1 is no longer available.

Though this poses some potential pitfalls, it also enables developers to define a single variable to do things like capture user inputs of varying data types, perform basic operations with them, and then overwrite the initial variable with a new instance, saving code effort. The code structure and use of the user inputs must be identical in this instance, but it simplifies otherwise complex solutions.