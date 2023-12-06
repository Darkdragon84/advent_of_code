use advent::util::io::file_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::iter;
use std::ops::Range;

const INPUT_FILE: &str = "data/p2023_05.txt";

#[derive(Debug, Hash)]
struct GardenRange {
    dst_range: Range<usize>,
    src_range: Range<usize>,
}

impl GardenRange {
    pub fn new(    dst_start: usize,        src_start: usize,        len: usize) -> Self {
            Self { dst_range: dst_start..(dst_start + len), src_range: src_start..(src_start + len)}
        }
    pub fn src_contains(&self, value: &usize) -> bool {
        self.src_range.contains(value)
    }
    pub fn dst_contains(&self, value: &usize) -> bool {
        self.dst_range.contains(value)
    }
}
#[derive(Debug)]
struct GardenMap {
    source: String,
    destination: String,
    map: HashMap<usize, usize>,
}
#[derive(Debug)]
struct GardenMapError;

impl Display for GardenMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "key and value sets are not equal")
    }
}

impl Error for GardenMapError {}

impl GardenMap {
    pub fn new(source: String, destination: String) -> Self {
        let map: HashMap<usize, usize> = HashMap::new();
        Self {
            source,
            destination,
            map,
        }
    }
    pub fn update(&mut self, range: GardenRange) {
        self.map.extend(range.map())
    }

    pub fn get<'b>(&'b self, source: &'b usize) -> &'b usize {
        self.map.get(source).unwrap_or(source)
    }

    pub fn check(&self) -> Result<(), GardenMapError> {
        let key_set: HashSet<&usize> = self.map.keys().clone().collect();
        let value_set: HashSet<&usize> = self.map.values().clone().collect();
        if key_set != value_set {
            return Err(GardenMapError);
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
struct MapCollection {
    name_to_map: HashMap<String, GardenMap>,
}

impl MapCollection {
    pub fn from_maps(maps: Vec<GardenMap>) -> Self {
        let name_to_map = HashMap::from_iter(maps.into_iter().map(|m| (m.source.clone(), m)));
        Self { name_to_map }
    }

    pub fn from_lines(lines: &Vec<String>) -> Self {
        let re_src_dst = Regex::new(r"(?<src>\w+)-to-(?<dst>\w+) map:").expect("not a valid regex");
        let re_rng =
            Regex::new(r"(?<dst>\d+)\s+(?<src>\d+)\s+(?<len>\d+)").expect("not a valid regex");
        let mut maps: Vec<GardenMap> = Vec::new();
        let mut mapopt: Option<&mut GardenMap> = None;

        for line in lines {
            if let Some(c) = re_src_dst.captures(&line.as_str()) {
                let (src_name, dst_name) = (
                    c.name("src").expect("src not found").as_str(),
                    c.name("dst").expect("dst not found").as_str(),
                );
                maps.push(GardenMap::new(src_name.to_string(), dst_name.to_string()));
                mapopt = maps.last_mut();
            } else if let Some(c) = re_rng.captures(&line.as_str()) {
                let (src_start, dst_start, len) = (
                    c.name("src")
                        .expect("src not found")
                        .as_str()
                        .parse::<usize>()
                        .expect("not a number"),
                    c.name("dst")
                        .expect("dst not found")
                        .as_str()
                        .parse::<usize>()
                        .expect("not a number"),
                    c.name("len")
                        .expect("len not found")
                        .as_str()
                        .parse::<usize>()
                        .expect("not a number"),
                );

                if let Some(ref mut map) = mapopt {
                    map.update(GardenRange {
                        dst_start,
                        src_start,
                        len,
                    });
                }
            }
        }
        // dbg!(&maps);
        Self::from_maps(maps)
    }

    pub fn get<'a>(
        &'a self,
        value: &'a usize,
        source: &String,
        destination: &String,
    ) -> Option<&'a usize> {
        let mut src = source;
        let mut val = value;
        loop {
            let mapopt = self.name_to_map.get(src);
            match mapopt {
                None => break None,
                Some(ref map) => {
                    val = map.get(&val);
                    src = &map.destination;
                    if map.destination == *destination {
                        break Some(val);
                    }
                }
            }
        }
    }

    pub fn get_map(&self, source: String) -> Option<&GardenMap> {
        self.name_to_map.get(&source)
    }
}

fn main() {
    let re_seed = Regex::new(r"\d+").expect("not a valid regex");
    let mut lines: Vec<String> = file_lines(INPUT_FILE)
        .filter_map(|line| line.ok())
        .filter(|line| line.len() > 0)
        .collect();

    let seeds = lines.remove(0);
    let seeds: Vec<usize> = re_seed
        .find_iter(&seeds.as_str())
        .map(|m| m.as_str().parse::<usize>().expect("not a valid number"))
        .collect();
    let collection = MapCollection::from_lines(&lines);
    println!("created collection");
    // for map in collection.name_to_map.values() {
    //     println!("{} -> {}: {:?}", map.source, map.destination, map.check())
    // }

    let src = "seed".to_string();
    let dst = "location".to_string();
    let locations: Vec<&usize> = seeds
        .iter()
        .map(|seed| collection.get(seed, &src, &dst).expect("couldn't map seed"))
        .collect();
    for (seed, location) in iter::zip(&seeds, &locations) {
        println!("{seed} -> {location}");
    }
    println!(
        "min location: {}",
        locations.iter().min().expect("no elements")
    );
}
