use std::{io, process::exit};


pub fn input() -> String {
    let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn trimmed() -> String {
    input().trim().to_string()
}

pub fn as_u8(failed: &str) -> usize{
    loop{
    match trimmed().parse() {
        Ok(i) => return i,
        Err(_) => {
            println!("{}",failed);
            continue;
        }
    }
    }
}

pub fn get_tool_or_quit(failed: &str) -> usize{
    loop {
        let input = trimmed().to_lowercase();
        if input == "q" || input == "quit" {
            exit(0)
        }
        match input.parse() {
            Ok(i) => return i,
            Err(_) => {
                println!("{}",failed);
                continue;
            }
        }

    }
}