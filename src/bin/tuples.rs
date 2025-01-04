#![allow(unused)]

fn main() {
    let tuple = (true, 3, 'b');
    // destructuring tuple
    let (x, y, z) = tuple;

    //ignorinng some values
    let (x, _, _) = tuple;

    println!("this is a tuple {}", { tuple.1 });
}
