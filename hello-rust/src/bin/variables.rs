#![allow(unused)]

fn main() {
    // Declare an immutable variable `x` with type `i32`
    let x: i32 = -123; 
    
    // x += 1; // This will cause an error because `x` is immutable

    // Declare a mutable variable `y`
    let mut y: i32 = 456; 

    y += 1; // Now `y` is 457

    println!("x: {}, y: {}", x, y);

    // Type inference allows `z` to be inferred as default `i32`
    let z = -123; 

    // Unit type, similar to `void` in other languages
    let w: ()= ();

    // const creates a constant (not a variable).
    // Must include a type (u32 here).
    // Cannot be mut — always immutable.
    const NUM: u32 = 1; 
    // Shadowing the previous `x` variable
    let x: i32 = -1; 
    // Shadowing `x` again with a different type
    let x: bool = true; 
    //Vec<_> means: let the compiler infer the type of elements (here, i32).
    //Rust sees you're creating a vector of integers → Vec<i32>
    let v: Vec<_> = vec![1, 2, 3];
}
