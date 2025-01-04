#![allow(unused)]

use std::vec;

fn main() {
    let x = 2;
    // x = 3;
    let mut y = 3;
    y += 1;

    const NUM: u32 = 1;
    let x = 2;
    let x = true;

    let v: Vec<_> = vec![1, 2, 3];

    // unsigned integer
    let u0: u8 = 1;
    let u1: u16 = 1;
    let u2: u32 = 1;
    let u3: u64 = 1;
    let u4: u128 = 1;

    // floats
    let f0: f32 = 0.34;

    // boolean
    let yes: bool = true;
    let no = false;

    // character
    let emoji = '😻';
}
