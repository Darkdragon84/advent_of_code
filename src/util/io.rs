use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;

pub fn file_buffer<P>(path: P) -> Result<BufReader<File>, Error>
where
    P: AsRef<Path>,
{
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn file_lines<P>(path: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
    let buf = file_buffer(path).expect("file not found");
    buf.lines()
}
