#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

fn if_statement() {
    // variable declaration
    let temp = 35;

    if temp > 30 {
        println!("really hot outside!");
    } else if temp < 10 {
        println!("really cold outside!");
    } else {
        println!("temperature is ok");
    }

    // if statements can be inline
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    println!(
        "it is {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "ok"
        }
    );

    // nested if statements
    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "ok"
        }
    )
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        // resumes executing the loop without running anything after this line "continue;"
        if x == 64 {
            continue;
        }

        println!("x is equal to {}", x)
    }

    let mut y = 1;
    // while true
    loop {
        y *= 2;

        println!("y is equal to {}", y);

        // break will completely break out of whatever current loop you are in
        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    // .. is a range
    for x in 1..11 {
        println!("x is equal to {}", x)
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 7;

    // rust is smart enough to yell at you for not covering all values, therefore the "_"
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid",
    };

    println!("the country with code {} is {}", country_code, country)
}

// use rand::Rng;
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked,
}

fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => entry.push_str(&input.trim_end()),
                    Err(_) => continue,
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}

fn main() {
    if_statement();
    while_and_loop();
    for_loop();
    match_statement();
    combination_lock();
}
