use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let _args: Vec<String> = std::env::args().collect();
    let filepath = _args[1].clone();

    println!("{}", filepath);
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}