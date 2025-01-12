#![allow(unused)]

#[derive(Debug)]
enum MathError{
    DivByZero,
    Other,
}
fn divide (x: u32, y:u32) -> Result<u32, MathError> {
    if y == 0 {
        return Err(MathError::DivByZero);
    }
   Ok(x/y)
}
fn main() {
    // error
    // panic!("crash");

    let arr = [1,2,3];

    let x: Option<&i32 > = arr.get(1);
    match x  {
        Some(val) => println!("value is {val}"),
        None => println!("none"),
    }

    let x = 1;
    let y = 0;
    // z type will be u32 or type MathError
    let z = divide(x,y);
    match z {
       Ok(val) => println!("div = {val}"),
        Err(err) => println!("err = {:?}", {err})
    }
}
