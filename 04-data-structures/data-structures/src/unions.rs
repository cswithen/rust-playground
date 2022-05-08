union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value")
            }
            IntOrFloat { f } => {
                // will also process a i value as f ; using a reinterpret
                println!("value = {}", f)
            }
        }
    }
}

pub fn unions() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat { i: 5 });
}
