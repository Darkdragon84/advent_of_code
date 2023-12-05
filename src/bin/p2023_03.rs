use advent::util::{io::file_lines, ranges_overlap};
use ascii::AsciiString;
use regex::{Match, Regex};
use std::cmp::{max, min};

const INPUT_FILE: &str = "data/p2023_03.txt";

fn extract_numbers(prev_line: &Matches, cur_line: &Matches, next_line: &Matches) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    assert_eq!(prev_line.line.len(), cur_line.line.len());
    assert_eq!(next_line.line.len(), cur_line.line.len());

    for num_match in cur_line.num_matches.iter() {
        // check if there is a symbol before or after the number
        let number = num_match.as_str().parse().expect("not a valid number");
        let (start, end) = (num_match.start(), num_match.end());
        if (start > 0 && cur_line.line[start - 1] != b'.')
            || (end < cur_line.line.len() - 1 && cur_line.line[end] != b'.')
        {
            println!("==============");
            println!("{}: {number}", cur_line.line);
            numbers.push(number);
        }
        let start = max(start as i32 - 1, 0) as usize;
        let end = min(end + 1, cur_line.line.len());
        let num_range = (&start, &end);
        if prev_line
            .symbol_matches
            .iter()
            .any(|sym_match| ranges_overlap((&sym_match.start(), &sym_match.end()), num_range))
            || next_line
                .symbol_matches
                .iter()
                .any(|sym_match| ranges_overlap((&sym_match.start(), &sym_match.end()), num_range))
        {
            println!("==============");
            println!(
                "{}\n{}: {number}\n{}",
                prev_line.line, cur_line.line, next_line.line
            );
            numbers.push(number);
        }
    }
    numbers
}

fn extract_gear_ratios(prev_line: &Matches, cur_line: &Matches, next_line: &Matches) -> Vec<u32> {
    let mut ratios: Vec<u32> = Vec::new();

    for gear_match in cur_line.symbol_matches.iter().filter(|m| m.as_str() == "*") {
        let start = max(gear_match.start() as i32 - 1, 0) as usize;
        let end = min(gear_match.end() + 1, cur_line.line.len());
        let gear_range = (&start, &end);

        // chain number matches of all 3 lines!
        let surrounding_numbers = prev_line
            .num_matches
            .iter()
            .chain(
                next_line
                    .num_matches
                    .iter()
                    .chain(cur_line.num_matches.iter()),
            )
            .filter(|num_match| ranges_overlap((&num_match.start(), &num_match.end()), gear_range))
            .map(|num_match| num_match.as_str().parse::<u32>().expect("not a number"))
            .collect::<Vec<_>>();

        if surrounding_numbers.len() == 2 {
            println!("==============");
            println!(
                "{}\n{}: {} -> ({} * {} = {})\n{}",
                prev_line.line,
                cur_line.line,
                gear_match.as_str(),
                surrounding_numbers[0],
                surrounding_numbers[1],
                surrounding_numbers[0] * surrounding_numbers[1],
                next_line.line
            );
            ratios.push(surrounding_numbers[0] * surrounding_numbers[1]);
        }
    }
    ratios
}

pub struct Matches<'a, 'h> {
    line: &'a AsciiString,
    num_matches: Vec<Match<'h>>,
    symbol_matches: Vec<Match<'h>>,
}

pub fn get_regex_matches(lines: &Vec<AsciiString>) -> Vec<Matches> {
    let re_num = Regex::new(r"\d+").expect("not a valid regex");
    let re_symbol = Regex::new(r"[^\.\d]").expect("not a valid regex");
    let mut matches = Vec::new();

    for line in lines {
        let num_matches = Vec::from_iter(re_num.find_iter(line.as_str()));
        let symbol_matches = Vec::from_iter(re_symbol.find_iter(line.as_str()));
        matches.push(Matches {
            line,
            num_matches,
            symbol_matches,
        })
    }
    matches
}

fn main() {
    let mut lines: Vec<AsciiString> = file_lines(INPUT_FILE)
        .map(|line| AsciiString::from_ascii(line.expect("not a line")).expect("not ascii"))
        .collect();
    let line_length = lines.iter().next().expect("no first line").len();
    let empty =
        AsciiString::from_ascii(String::from_iter(std::iter::repeat('.').take(line_length)))
            .expect("not ascii");

    lines.insert(0, empty.clone());
    lines.push(empty.clone());
    // dbg!(&lines);

    let matches = get_regex_matches(&lines);

    let mut number_sum = 0u32;
    for win in matches.windows(3) {
        if let [prev_line, cur_line, next_line] = win {
            let line_numbers = extract_numbers(prev_line, cur_line, next_line);
            number_sum += line_numbers.iter().sum::<u32>();
        }
    }

    let mut gear_ratio_sum = 0u32;

    for win in matches.windows(3) {
        if let [prev_line, cur_line, next_line] = win {
            let line_ratios = extract_gear_ratios(prev_line, cur_line, next_line);
            gear_ratio_sum += line_ratios.iter().sum::<u32>();
        }
    }
    println!("number sum: {number_sum}");
    println!("gear ratio sum: {gear_ratio_sum}");
}
