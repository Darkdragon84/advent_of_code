use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const INPUT_FILE: &str = "data/p2023_02.txt";
const RE: &str = r"(\d+)\s+(red|green|blue)";

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

fn get_str_to_enum() -> HashMap<&'static str, Color> {
    HashMap::from([
        ("red", Color::Red),
        ("green", Color::Green),
        ("blue", Color::Blue),
    ])
}

#[derive(Debug)]
pub struct ColorCounter {
    color_to_ct: HashMap<Color, Vec<u32>>,
}

impl ColorCounter {
    pub fn new() -> ColorCounter {
        let mut color_to_ct: HashMap<Color, Vec<u32>> = HashMap::new();

        color_to_ct.insert(Color::Red, vec![]);
        color_to_ct.insert(Color::Green, vec![]);
        color_to_ct.insert(Color::Blue, vec![]);
        ColorCounter { color_to_ct }
    }
    pub fn counts(&self, color: &Color) -> &Vec<u32> {
        self.color_to_ct.get(color).expect("color not found")
    }
    pub fn mut_counts(&mut self, color: &Color) -> &mut Vec<u32> {
        self.color_to_ct.get_mut(color).expect("color not found")
    }
    pub fn update(&mut self, color: &Color, count: u32) {
        self.mut_counts(color).push(count);
    }
    pub fn max(&self, color: &Color) -> &u32 {
        self.counts(&color).iter().max().unwrap_or(&0)
    }
}
fn process_game(line: &String) -> (u32, u32, u32) {
    let str_to_enum = get_str_to_enum();
    let mut color_counter = ColorCounter::new();
    let re = Regex::new(RE).expect("not a valid regex");
    let (_, draws) = line.split_once(": ").expect("no ': ' found");

    for (_, [count, color]) in re.captures_iter(draws).map(|c| c.extract::<2>()) {
        color_counter.update(
            str_to_enum.get(color).expect("color not found"),
            count.parse().expect("not a valid number"),
        );
    }
    (
        color_counter.max(&Color::Red).clone(),
        color_counter.max(&Color::Green).clone(),
        color_counter.max(&Color::Blue).clone(),
    )
}

fn main() -> Result<(), Error> {
    let (r_max, g_max, b_max) = (12u32, 13u32, 14u32);

    let mut id_sum = 0u32;
    let mut power_sum = 0u32;
    for (id, line) in BufReader::new(File::open(INPUT_FILE)?).lines().enumerate() {
        let (r, g, b) = process_game(&line?);
        let valid = r <= r_max && g <= g_max && b <= b_max;
        if valid {
            id_sum += (id + 1) as u32;
        }
        power_sum += r * g * b;
        println!("{id}: max counts: r {r:2}, g {g:2}, b {b:2}, valid: {valid}");
    }
    println!("game id sum: {id_sum}");
    println!("power sum: {power_sum}");
    Ok(())
}
