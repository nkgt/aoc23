use std::{error::Error, fs};

fn part1(input: &str) -> u32 {
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

    total
}

fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;
    
    for line in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        
        let (_, rolls) = line.split_once(":").unwrap();

        for pair in rolls.split([',', ';']) {
            let (n, color) = pair.trim_start().split_once(" ").unwrap();
            let count = n.parse::<u32>().unwrap();

            match color {
                "red"   => if count > min_red   { min_red = count; },
                "green" => if count > min_green { min_green = count; },
                "blue"  => if count > min_blue  { min_blue = count; },
                _ => panic!("Wrong input"),
            }
        }

        total += min_blue * min_red * min_green;
    }

    total
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}

