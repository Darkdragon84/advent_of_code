use regex::Regex;
use std::collections::HashMap;
use advent::util::io::file_lines;

const INPUT_FILE: &str = "data/p2023_01.txt";
const DIG_STR: &str = "one|two|three|four|five|six|seven|eight|nine";

fn get_str_to_digit() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ])
}

fn extract_number_from_digits(digits: &Vec<u32>) -> u32 {
    if digits.len() == 0 {
        return 0;
    }
    let first = digits.iter().next().unwrap();
    let last = digits.iter().rev().next().unwrap();
    let number: u32 = format!("{}{}", first, last).to_string().parse().unwrap();
    // println!("{:?} -> {}", digits, number);
    number
}

fn reverse_string(s: &str) -> String {
    String::from_iter(String::from(s).chars().rev())
}

fn get_line_value2(line: &String) -> u32 {
    let re_fwd_str = format!(r"(\d|{})", DIG_STR);
    let re_rev_str = format!(r"(\d|{})", reverse_string(DIG_STR));
    let re_fwd = Regex::new(re_fwd_str.as_str()).expect("not a valid regex expression");
    let re_rev = Regex::new(re_rev_str.as_str()).expect("not a valid regex expression");
    let str_to_digit = get_str_to_digit();
    let first = re_fwd.find(line).expect("no fwd match found").as_str();
    let last = reverse_string(
        re_rev
            .find(reverse_string(line).as_str())
            .expect("no rev match found")
            .as_str(),
    );
    let last = last.as_str();

    // turn spelled out digits into digits str
    let first = str_to_digit.get(first).unwrap_or(&first);
    let last = str_to_digit.get(last).unwrap_or(&last);

    let number: u32 = format!("{}{}", first, last)
    .parse()
    .expect("not a valid number");

    number
}

fn get_line_value1(line: &String) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    extract_number_from_digits(&digits)
}

fn main() {
    let mut sum1 = 0u32;
    for line in file_lines(INPUT_FILE) {
        sum1 += get_line_value1(&line.expect("empty line"));
    }
    println!("sum1: {}", sum1);

    let mut sum2 = 0u32;
    for line in file_lines(INPUT_FILE){
        sum2 += get_line_value2(&line.expect("empty line"));
    }
    println!("sum2: {}", sum2);
}
