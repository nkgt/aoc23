use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut valid = true;
        let (entry, rolls) = line.split_once(":").unwrap();

        for pair in rolls.split([',', ';']) {
            let (n, color) = pair.trim_start().split_once(" ").unwrap();
            let count = n.parse::<u32>().unwrap();

            match color {
                "red"   => if count > red_limit   { valid = false; break; },
                "green" => if count > green_limit { valid = false; break; },
                "blue"  => if count > blue_limit  { valid = false; break; },
                _ => panic!("Wrong input"),
            }
        }

        if valid {
            let (_, id) = entry.split_once(" ").unwrap(); 
            total += id.parse::<u32>().unwrap();
        }
    }

    println!("{}", total);

    Ok(())
}

