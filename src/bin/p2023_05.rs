use advent::util::io::file_lines;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter;
use std::ops::Range;

const INPUT_FILE: &str = "data/p2023_05_test.txt";

#[derive(Debug)]
struct GardenRange {
    dst_start: usize,
    src_start: usize,
    len: usize,
}

impl GardenRange {
    pub fn src_end(&self) -> usize {
        self.src_start + self.len
    }
    pub fn src_range(&self) -> Range<usize> {
        self.src_start..self.src_end()
    }
    pub fn dst_end(&self) -> usize {
        self.dst_start + self.len
    }
    pub fn dst_range(&self) -> Range<usize> {
        self.dst_start..self.dst_end()
    }
    pub fn map(&self) -> HashMap<usize, usize> {
        HashMap::from_iter(iter::zip(self.src_range(), self.dst_range()))
    }
}
#[derive(Debug)]
struct GardenMap {
    source: String,
    destination: String,
    map: HashMap<usize, usize>,
}

impl GardenMap {
    pub fn new(source: String, destination: String) -> Self {
        let map: HashMap<usize, usize> = HashMap::new();
        Self {
            source,
            destination,
            map,
        }
    }
    pub fn extend(&mut self, range: GardenRange) {
        self.map.extend(range.map())
    }

    pub fn get<'b>(&'b self, source: &'b usize) -> &'b usize {
        self.map.get(source).unwrap_or(source)
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
                println!("{}: {:?} -> {:?}", line, src_name, dst_name)
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
                    map.extend(GardenRange {
                        dst_start,
                        src_start,
                        len,
                    });
                }
                println!("{}: src: {src_start}, dst: {dst_start}, len: {len}", line);
            }
        }
        // dbg!(&maps);
        Self::from_maps(maps)
    }

    pub fn get<'a>(&'a self, value: &'a usize, destination: &String) -> Option<&'a usize> {
        let mut src = &"seed".to_string();
        loop {
            let mapopt = self.name_to_map.get(src);
            match mapopt {
                None => break None,
                Some(ref map) => {
                    let value = map.get(&value);
                    src = &map.destination;
                    if map.destination == *destination {
                        break Some(value);
                    }
                }
            }
        }
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
    let dst = "location".to_string();
    dbg!(&collection);
    for seed in seeds {
        println!("{seed} -> {:?}", collection.get(&seed, &dst));
    }
}