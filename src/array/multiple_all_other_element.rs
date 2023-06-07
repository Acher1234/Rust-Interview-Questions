use crate::utils::array_utils::create_random_array;
use std::vec::Vec;

pub fn main_function() {
    let random_array: Vec<i32> = create_random_array(4, 1, 100);
    let prefix_multiple = create_prefix_multiple(&random_array);
    let postfix_multiple = create_postfix_multiple(&random_array);
    let final_array: Vec<i32> = prefix_multiple
        .iter()
        .zip(postfix_multiple.iter())
        .map(|(x, y)| x * y)
        .collect();
    print!("Array: {:?}\n", random_array);
    print!("Final array: {:?}\n", final_array);
}

fn create_prefix_multiple(array: &Vec<i32>) -> Vec<i32> {
    let mut prefix_multiple: Vec<i32> = Vec::new();
    let mut multiple: i32 = 1;
    for i in 0..array.len() {
        prefix_multiple.push(multiple);
        multiple *= array[i];
    }
    return prefix_multiple;
}

fn create_postfix_multiple(array: &Vec<i32>) -> Vec<i32> {
    let mut postfix_multiple: Vec<i32> = Vec::new();
    let mut multiple: i32 = 1;
    for i in (0..(array.len())).rev() {
        postfix_multiple.push(multiple);
        multiple *= array[i];
    }
    postfix_multiple.reverse();
    return postfix_multiple;
}
