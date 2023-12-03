use std::{fs, error::Error};

fn part1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let mut temp = 0;

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

    total
}

fn part2(input: &str) -> u32 {
    let mut total = 0;
    let digit_words = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    for line in input.lines() {
        let mut temp = 0;

        'outer: for i in 0..line.len() {
            let substr = &line[i..];
            
            let c: char = substr.chars().next().unwrap();
            if c.is_digit(10) {
                temp += c.to_digit(10).unwrap() * 10;
                break;
            }

            for (i, digit) in digit_words.iter().enumerate() {
                if substr.starts_with(digit) {
                    temp += ((i + 1) * 10) as u32;
                    break 'outer;
                }
            }
        }

        'outer: for i in 0..line.len() {
            let last_pos = line.len() - i;
            let substr = &line[..last_pos];

            let c: char = substr.chars().last().unwrap();
            if c.is_digit(10) {
                temp += c.to_digit(10).unwrap();
                break;
            }

            for (i, digit) in digit_words.iter().enumerate() {
                if substr.ends_with(digit) {
                    temp += (i + 1) as u32;
                    break 'outer;
                }
            }
        }

        total += temp;
    }

    total
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
