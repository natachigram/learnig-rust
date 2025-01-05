#![allow(unused)]
#[derive(Debug)]

struct Lang {
    name: String,
    version: f32,
    author: String,
}

#[derive(Debug)]
struct Device {
    name: String,
    model: String,
    price: f32,
}
#[derive(Debug)]
struct System {
    lang: Lang,
    device: Device,
}

fn main() {
    let new_lang = Lang {
        name: "Rust".to_string(),
        version: 1.0,
        author: "natachi".to_string(),
    };

    println!("{:?}", new_lang);

    //3d struct
    let new_system = System {
        lang: new_lang,
        device: Device {
            name: "macbook".to_string(),
            model: "space20".to_string(),
            price: 24.5,
        },
    };

    println!("{:#?}", new_system);
}
