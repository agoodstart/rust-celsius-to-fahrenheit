use std::io::{self};
use ansi_term::Style;
mod converter;

fn main() {
    println!("{}", Style::new().bold().paint("Convert temperatures"));
    println!("Please choose one of the following:
    1: Celsius to Fahrenheit
    2: Fahrenheit to Celsius");

    loop {
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        match converter::to_num_i8(&choice) {
            Ok(num) => {
                match num == 1 || num == 2 {
                    true => {
                        break converter::convert(num);
                    },
                    false => println!("Please type either '1' or '2'")
                }
            },
            Err(_) => println!("Not a number"),
        }
    }

}