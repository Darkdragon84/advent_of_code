use advent::util::io::file_lines;
// use ascii::AsciiString;
use regex::Regex;
use std::collections::HashSet;
use std::io::Error;

const INPUT_FILE: &str = "data/p2023_04.txt";

fn num_winning_numbers(line: &String) -> u32 {
    let re = Regex::new(r"\d+").expect("not a valid regex");
    let (_, game) = line.split_once(": ").expect("no ':' found");
    let (winning, actual) = game.split_once("|").expect("no '|' found");
    let winning_numbers: HashSet<u32> = HashSet::from_iter(
        re.find_iter(winning)
            .map(|m| m.as_str().parse::<u32>().expect("not a valid number")),
    );
    let actual_numbers: HashSet<u32> = HashSet::from_iter(
        re.find_iter(actual)
            .map(|m| m.as_str().parse::<u32>().expect("not a valid number")),
    );
    winning_numbers
        .intersection(&actual_numbers)
        .collect::<Vec<_>>()
        .len() as u32
}

fn main() -> Result<(), Error> {
    let num_winning: Vec<u32> = file_lines(INPUT_FILE)?
        .map(|line| num_winning_numbers(&line.expect("no line found")))
        .collect();
    let score_sum: u32 = num_winning
        .iter()
        .map(|num_common| {
            if num_common > &0 {
                2u32.pow(num_common - 1)
            } else {
                0
            }
        })
        .sum();
    println!("score sum: {score_sum}");
    Ok(())
}
