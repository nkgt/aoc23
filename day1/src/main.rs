use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let mut total = 0;
    let mut temp;

    for line in input.lines() {
        temp = 0;

        for c in line.chars() {
            if c.is_digit(10) {
                temp += c.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                temp += c.to_digit(10).unwrap();
                break;
            }
        }

        total += temp;
    }

    println!("{}", total);

    Ok(())
}
