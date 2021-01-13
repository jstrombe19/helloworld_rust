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


