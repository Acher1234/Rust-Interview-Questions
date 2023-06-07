use super::super::Utils::utils;
use std::io;

fn main() {
    get_input_project();
}

fn get_input_project() -> i32 {
    let mut input = String::new();
    let mut given_num = guess_number();
    println!("You entered {}", given_num);
    given_num
}
