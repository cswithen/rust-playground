#[allow(dead_code)]
#[allow(unused_variables)]

// standard library to be used within the code
// use std::mem;

// global variables
const MEANING_OF_LIFE: u8 = 42; // this has no fixed address

// static variables
static mut Z: i32 = 123; // has a fixed address

fn operators() {
    // arithmetic

    let mut a = 2 + 3 * 4; // +-*/
    println!("a = {}", a);
    a = a + 1; // does not support -- or ++
    a -= 2; // -= += *= /= %=

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("a_cubed = {} ", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR, & AND, ^ XOR, ! NOR
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true, > <= >= ==
    println!("is pi less than 4, {}", pi_less_4);

    let x = 5;
    let x_is_5 = x == 5; // true
    println!("is x equal to 5, {}", x_is_5);
}

fn main() {
    operators();
    println!("global variable, {}", MEANING_OF_LIFE);

    unsafe {
        Z = 777;
        println!("static variable, {}", Z);
    }
}
