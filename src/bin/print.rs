#![allow(unused)]
#[derive(Debug)]

struct Lang {
    language: String,
    versions: String,
}

fn main() {
    let lang = "rust";
    println!("Hello, {}!", lang);
    println!("{}", lang);
    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);
    let lang = Lang {
        language: "Rust".to_string(),
        versions: "2018".to_string(),
    };

    println!("{:?}", lang);
    //print struct with line breaks
    println!("{:#?}", lang);
}
