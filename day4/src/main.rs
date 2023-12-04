use std::{error::Error, fs};

fn part1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning, tries) = numbers.split_once('|').unwrap();
        let mut wins = 0;

        for n in tries.split(' ').filter(|&x| x != "") {
            if winning.split(' ').filter(|&x| x != "").any(|x| x == n) {
                wins += 1;
            }
        }

        if wins > 0 {
            total += 2u32.pow(wins - 1);
        }
    }

    total
}

fn part2(input: &str) -> u32 {
    let line_count = input.lines().count();
    let mut cards_count = vec![1u32; line_count];

    for (i, line) in input.lines().enumerate() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning, tries) = numbers.split_once('|').unwrap();
        let mut wins = 0usize;

        for n in tries.split(' ').filter(|&x| x != "") {
            if winning.split(' ').filter(|&x| x != "").any(|x| x == n) {
                wins += 1;
            }
        }

        let min = i + 1;
        let max = i + wins + 1;

        for j in min..max {
            cards_count[j] += cards_count[i];
        }
    }

    cards_count.iter().fold(0, |a, e| a + e)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
