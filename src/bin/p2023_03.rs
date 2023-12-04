use advent::util::io::file_lines;
use ascii::{AsciiString, AsAsciiStr};
use regex::Regex;
use std::cmp::{max, min};

use std::collections::btree_map::Range;
use std::io::Error;
const INPUT_FILE: &str = "data/p2023_03_test.txt";

fn extract_numbers(
    prev_line: &Matches,
    cur_line: &Matches,
    next_line: &Matches,
) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    assert_eq!(prev_line.line.len(), cur_line.line.len());
    assert_eq!(next_line.line.len(), cur_line.line.len());

    for m in re_num.find_iter(cur_line.as_str()) {
        // check if there is a symbol before or after the number
        let number = m.as_str().parse().expect("not a valid number");
        let (start, end) = (m.start(), m.end());
        if (start > 0 && cur_line[start - 1] != b'.')
            || (end < cur_line.len() - 1 && cur_line[end] != b'.')
        {
            // println!("==============");
            // println!("{}: {number}", cur_line);
            numbers.push(number);
        }
        let start = max(start as i32 - 1, 0) as usize;
        let end = min(end + 1, cur_line.len());
        // check if there is a symbol around the number in the previous or next line
        if re_symbol.find(&prev_line.as_str()[start..end]) != None
            || re_symbol.find(&next_line.as_str()[start..end]) != None
        {
            // println!("==============");
            // println!("{}\n{}: {number}\n{}", prev_line, cur_line, next_line);
            numbers.push(number);
        }
    }
    numbers
}
// pub fn extract_numbers_ranges()
// pub fn extract_numbers_ranges()
pub struct Matches<'a> {
    line: &'a AsciiString,
    num_matches: Vec<(usize, usize)>,
    symbol_matches: Vec<(usize, usize)>,
}

pub fn get_regex_matches(lines: &Vec<AsciiString>) -> Vec<Matches> {

    let re_num = Regex::new(r"\d+").expect("not a valid regex");
    let re_symbol = Regex::new(r"[^\.\d]").expect("not a valid regex");
    let mut matches = Vec::new();

    for line in lines {
        let num_matches = Vec::from_iter(re_num.find_iter(line.as_str()).map(|m| (m.start(), m.end())));
        let symbol_matches = Vec::from_iter(re_symbol.find_iter(line.as_str()).map(|m| (m.start(), m.end())));
        matches.push(Matches{line, num_matches, symbol_matches})
    }
    matches
}

pub fn test_zip() {
    let a = vec![1, 2, 3, 4, 5, 6];
    let b = vec![4, 5, 6];
    for (x, y) in std::iter::zip(&a[..2], &b[1..]) {
        println!("{x}, {y}");
    }
}

fn main() -> Result<(), Error> {
    // test_zip();
    // return Ok(());

    let mut lines: Vec<AsciiString> = file_lines(INPUT_FILE)?
        .map(|line| AsciiString::from_ascii(line.expect("not a line")).expect("not ascii"))
        .collect();
    let line_length = lines.iter().next().expect("no first line").len();
    let empty =
        AsciiString::from_ascii(String::from_iter(std::iter::repeat('.').take(line_length)))
            .expect("not ascii");

    let matches = get_regex_matches(&lines);
    // lines.insert(0, empty.clone());
    // lines.push(empty.clone());

    for mtch in matches.iter() {
        println!("================================");
        println!("{}", &mtch.line);
        for (i_start, i_end) in mtch.num_matches.iter(){
            println!("{}: ({i_start}-{i_end})", &mtch.line[*i_start..*i_end]);
        }
        for (i_start, i_end) in mtch.symbol_matches.iter(){
            println!("{}: ({i_start}-{i_end})", &mtch.line[*i_start..*i_end]);
        }
    }

    // let mut sum = 0u32;
    // // dbg!(&lines);
    // for win in lines.windows(3) {
    //     if let [prev_line, cur_line, next_line] = win {
    //         let line_numbers = extract_numbers(prev_line, cur_line, next_line);
    //         sum += line_numbers.iter().sum::<u32>();
    //     }
    // }
    // println!("sum: {sum}");
    // regex_test();
    Ok(())
}
