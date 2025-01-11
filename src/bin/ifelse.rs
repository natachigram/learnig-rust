#![allow(unused)]

fn main() {
    let x = 5;
    if x > 2 {
        println!("1 is greater than 2");
    } else {
        println!("1 is not greater than 2");
    }

    let y = if 2 + x == 0 {
        println!("ZERO");
    } else {
        println!("NOT ZERO");
    };
}
