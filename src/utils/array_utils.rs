use super::random_number::generate_random_number;

pub fn create_random_array(number_element: i32, start: i32, end: i32) -> Vec<i32> {
    let mut random_array: Vec<i32> = Vec::new();
    for _ in 0..number_element {
        random_array.push(generate_random_number(start, end));
    }
    return random_array;
}
