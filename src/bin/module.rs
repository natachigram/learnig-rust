#![allow(unused)]

use hello_rust::test_mod;

fn main() {
    test_mod:: print();
    test_mod::another_mud::print_another_mod();

    let javascript = test_mod::another_mud::Lang{
        name: "Javascript".to_string(),
        version: 0,
    };

    println!("first language was {:?}", {javascript.name});

    test_mod::call_external();
    test_mod::another_mud::call_external();
}