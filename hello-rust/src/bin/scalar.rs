#![allow(unused)]

use std::u32;

fn main() {
    // Scalar types in Rust are the most basic data types.
    // single vlaue
    // building blocks for more complex types
    // integers
    // signed integers
    // -(2^n - 1) to (2^n - 1)
    let i0: i8 = 0; // 8-bit signed integer
    let i1: i16 = 1; // 16-bit signed integer
    let i2: i32 = 1; // 32-bit signed integer
    let i3: i64 = 1; // 64-bit signed integer
    let i4: i128 = 1; // 128-bit signed integer
    let i5: isize = 1; // platform-dependent signed integer (32 or 64 bits)
                       //unsigned integers
                       // 0 to (2^n - 1)
    let u0: u8 = 1; // 8-bit unsigned integer
    let u1: u16 = 1; // 16-bit unsigned integer
    let u2: u32 = 1; // 32-bit unsigned integer
    let u3: u64 = 1; // 64-bit unsigned integer
    let u4: u128 = 1; // 128-bit unsigned integer
    let u5: usize = 1; // platform-dependent unsigned integer (32 or 64 bits)
                       // floats
    let f0: f32 = 1.0; // 32-bit floating point
    let f1: f64 = 1.0; // 64-bit floating point
                       // boolean

    let b: bool = true; // default type for floating point numbers
                        // characters

    let c: char = 'c'; // 4-byte Unicode character
    let e: char = '😊'; // Unicode character, can be an emoji or any other character
    //type conversion
    let i: i32= 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    //min and max values
    let min_i: i32 = i32::MIN; // minimum value for i32
    let max_i: i32 = i32::MAX; // maximum value for i32
    let min_u: u32 = u32::MIN; // minimum value for u32

    println!("i32 min:{min_i}");
    println!("i32 max:{max_i}");

    let min_char: char = char::MIN; // minimum value for char
    let max_char: char = char::MAX; // maximum value for char

    println!("char min:{min_char}");
    println!("char max:{max_char}");

    //Overflow
    let mut u: u32 = u32::MAX; // maximum value for u32
    u += 1; // This will cause an overflow, wrapping around to 0
    println!("u32 after overflow: {}", u); // prints 0
    // Note: In Rust, integer overflow in debug mode will panic, but in release mode, it will wrap around.

    //checked_add - returns None if overflow occurs
    let u = u32::checked_add(u32::MAX, 1);
    println!("checked_add result: {:?}", u); // prints None, as it overflows
    //wrapping_add - wraps around on overflow
     // This will not panic, it will wrap around to 0
    let u = u32::wrapping_add(u32::MAX, 1);
     println!("wrapping_add result: {:?}", u)

}
