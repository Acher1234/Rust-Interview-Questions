use crate::utils::array_utils::create_random_array;

pub fn find_smallest_windows_to_be_sorted() {
    let mut base_array = create_random_array(10, 0, 1000);
    println!("{:?}", base_array);
    let mut min_value_seen = 10000;
    let mut max_value_seen = -10000;
    let mut rigth_value_index = 0;
    let mut left_value_index = 0;
    (left_value_index, max_value_seen) = find_left_bound_and_his_last_value(&base_array);
    (rigth_value_index, min_value_seen) =
        find_rigth_bound_and_his_last_value(&base_array, &max_value_seen);
    println!("{} {}", left_value_index, rigth_value_index);
}

fn find_left_bound_and_his_last_value(base_array: &Vec<i32>) -> (usize, i32) {
    let mut max_value_seen = -10000;
    let mut left_value_index = 0;
    for (index, value) in base_array.iter().enumerate() {
        if *value > max_value_seen {
            max_value_seen = *value;
        } else {
            left_value_index = index;
            break;
        }
    }
    return (left_value_index, max_value_seen);
}

fn find_rigth_bound_and_his_last_value(
    base_array: &Vec<i32>,
    max_value_of_left_bound: &i32,
) -> (usize, i32) {
    let mut rigth_value_index = 0;
    let mut min_value_seen = 10000;
    for (index, value) in base_array.iter().rev().enumerate() {
        if *value < min_value_seen && *max_value_of_left_bound < *value {
            min_value_seen = *value;
        } else {
            rigth_value_index = base_array.len() - index - 1;
            break;
        }
    }
    return (rigth_value_index, min_value_seen);
}
