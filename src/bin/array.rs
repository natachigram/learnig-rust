#![allow(unused)]

fn main() {
    let arr = [1, 2, 3];
    println!("this ia an array  {}", arr[2]);

    let mut arr = [0, 4, 1];
    arr[0] = 90;

    println!("made an upadte to the array {}", arr[0]);

    let all_arr = [2; 20];
    println!("new array {:?}", all_arr);
    //slice
    let nums = [5; 4];
    let slice = &nums[0..2];

    println!("slice {:?}", slice);
}
