#![allow(unused)]

fn main() {
    let msg: String = String::from("Hello, Rust!");
    let len = msg.len();
    println!("msg: {:?}, len: {:?}", msg, len);

    // slice
    let s = &msg[0..5];
    let new_msg: &str = "Hello, people";

    println!("s: {:?}, new_msg: {:?}", s, new_msg);

    // multiple lines
    let multi = r#"
    my name is rust,
    come learn 
    with me
    🤭
    "#;

    println!("multi: {}", multi);
    println!("{multi}");
}
