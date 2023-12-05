use advent::util::io::file_lines;
use std::iter;
use regex::Regex;
use std::collections::HashSet;

const INPUT_FILE: &str = "data/p2023_04.txt";

fn num_winning_numbers(line: &String) -> usize {
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
        .len()
}

fn total_number_of_cards(game_to_num_winning: &Vec<usize>) -> u32 {
    let mut card_counts: Vec<u32> = iter::repeat(1u32).take(game_to_num_winning.len()).collect();
    for (game_id, num_winning) in game_to_num_winning.iter().enumerate() {
        for offset in 0..*num_winning {
            card_counts[game_id + 1 + offset] += card_counts[game_id];
        }
    }
    card_counts.iter().sum()
}

fn main() {
    let game_to_num_winning: Vec<usize> = file_lines(INPUT_FILE)
        .map(|line| num_winning_numbers(&line.expect("file not found")))
        .collect();
    let score_sum: u32 = game_to_num_winning
        .iter()
        .map(|num_common| {
            if num_common > &0 {
                2u32.pow((num_common - 1) as u32)
            } else {
                0
            }
        })
        .sum();
    println!("score sum: {score_sum}");
    println!("total number of cards: {}", total_number_of_cards(&game_to_num_winning));
}
