#![allow(unused)]

fn main() {
    //loop
    let mut counter = 0;
    loop {
        println!("loop {counter}");
        if counter == 5 {
            break;
        }
        counter += 1;
    }

    //while loop
    let mut counter = 0;
    while counter <= 7 {
        println!("while {counter}");
        counter += 1;
    }

    //for loop
    for counter in 0..4 {
        println!("THis is a for loop {counter}");
    }

    //for loop array
    let arr = [1, 2, 3, 4, 5];
    for a in arr {
        println!("Array {a}");
    }

    let lenght = arr.len();
    for i in 0..lenght {
        println!("Array {}", { arr[i] });
    }

    let z = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("z = {:?}", { z });

    //label loop
    'outer: for i in 0..4 {
        'inner: for j in 0..4 {
            if i == 2 && j == 3 {
                break 'outer;
            }
            println!("i = {i}, j = {j}");
        }
    }
}
