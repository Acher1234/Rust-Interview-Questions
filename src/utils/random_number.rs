use rand::Rng;

pub fn generate_random_number(start: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(start..end);
    return random_number;
}
