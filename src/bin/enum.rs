#![allow(unused)]
#[derive(Debug, PartialEq)]

enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}
fn main() {
    // enum
    let color = Color::Red;
    let color = Color::Green;
    let color = Color::Blue;
    let color = Color::Rgba(0, 0, 24, 0.0);
    let color = Color::Hex("#ffffff".to_string());
    let color = Color::Hsl { h: 5, s: 67, l: 0 };

    println!("{:?}", color);

    // parsalEq
    println!("{}", Color::Red == Color::Red);
    println!("{}", Color::Red == Color::Green);

    //option
    let y: Option<i32> = Some(5);
    print!("{:?}", y);

    // result
    let x: Result<i32, &str> = Ok(-3);
}
