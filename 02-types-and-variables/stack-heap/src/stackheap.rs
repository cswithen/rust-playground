#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin();
    // p2 is setup on the heap as a pointer
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 16 bytes

    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 bytes, because it is a pointer

    let p3 = *p2;
    println!("p3 x-value = {}, p3 y-value = {}", p3.x, p3.y); // (0,0)
}
