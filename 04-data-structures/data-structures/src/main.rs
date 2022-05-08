mod enums;
mod unions;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };

    println!("point P is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };

    let myline = Line { start: p, end: p2 };

    println!(
        "myLine starts at ({},{}), and ends at ({},{})",
        myline.start.x, myline.start.y, myline.end.x, myline.end.y
    )
}

fn main() {
    structures();
    enums::enums();
    unions::unions();
}
