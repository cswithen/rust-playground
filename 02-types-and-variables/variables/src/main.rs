#[allow(dead_code)]
#[allow(unused_variables)]
// standard library to be used within the code
use std::mem;

fn fundamental_data_types() {
    // number
    let a: u8 = 123; // u = unsigned, 8 bits, 0 -- 255
    println!("a = {}", a); // immutable (inside rust all variables are immutable)

    // mutable due to the "mut" keyword
    let mut b: i8 = 0; // i = signed, -128 -- 127
    println!("b = {} before", b);

    b = 42;
    println!("b = {} after", b);

    // variable type can be set using inference
    let mut c = 123456789; // i32 = 32 bits = 4 bytes
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after", c);

    // u8, u16, u32, u64, i8, i16, ...

    // usize and isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z); //usize

    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    // character
    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // f32 and f64 (floats) "IEEE754" signed!
    let e: f64 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    // boolean
    let g: bool = false; // true, false
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));
}

fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
}

fn main() {
    fundamental_data_types();
    scope_and_shadowing();
}
