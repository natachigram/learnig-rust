
use super::external;
pub fn call_external(){
    external::ext();
}
pub fn print(){
    println!("this is a module");
}

pub fn print_new_mod(){
    another_mud:: print_another_mod();
}
pub mod another_mud;