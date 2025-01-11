#![allow(unused)]
enum Lang {
    Rust,
    Go,
    Python,
    Java,
}
fn main() {
    // match
    let x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Any"),
    }

    //multiple cases
    match x {
        1 | 2 | 3 => println!("One, Two, or Three"),
        _ => println!("Any"),
    }
    //match btw ranges
    match x {
        1..=6 => println!("One to Six"),
        _ => println!("Any"),
    }

    //know exact match
    match x {
        i @ 1..=6 => println!("matched {i}"),
        _ => println!("Any"),
    }

    let lang = Lang::Rust;
    let lang_print = match lang {
        Lang::Rust => "Rust",
        Lang::Go => "Go",
        Lang::Python => "Python",
        Lang::Java => "Java",
        _ => "?",
    };

    println!("Lang is {lang_print}");
}
