/// Prompt for user input and return input as a string
pub fn prompt(msg: &str) -> String {
    println!("{msg}");

    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(num) => num,
        Err(_) => panic!("Could not read input"),
    };

    return input;
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}