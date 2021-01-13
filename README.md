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