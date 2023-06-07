use std::io;

pub fn guess_number(error_string: Option<String>) -> i32 {
    let mut input = String::new();
    let mut given_num: i32 = 0;
    let error_sentence = error_string.unwrap_or("Please enter a number".to_string());
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        given_num = match input.trim().parse() {
            Ok(num) => num,
            Err(_x) => {
                println!("{}", error_sentence);
                input.clear();
                continue;
            }
        };
        break;
    }
    return given_num;
}
