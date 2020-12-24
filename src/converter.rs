use std::io::{self};

pub fn convert(c: i8) {
    println!("Please enter the amount you want to convert: ");

    loop {
        let mut amount = String::new();

        io::stdin().read_line(&mut amount)
        .expect("Failed to read line");
    
        match to_num_f64(&amount) {
            Ok(num) => {
                match c {
                    1 => break println!("{celsius} Celsius is {fahrenheit} Fahrenheit", celsius = &num, fahrenheit = (&num * 9.0/5.0) + 32.0),
                    2 => break println!("{fahrenheit} Fahrenheit is {celsius} Celsius", fahrenheit = &num, celsius = (&num - 32.0) * 5.0/9.0),
                    _ => println!("Something went wrong...")
                }
            },
            Err(_) => println!("Couldn't read amount")
        }
    }
}

pub fn to_num_f64(x: &str) -> Result<f64, std::num::ParseFloatError> {
    x.trim().parse()
}

pub fn to_num_i8(x: &str) -> Result<i8, std::num::ParseIntError> {
    x.trim().parse()
}