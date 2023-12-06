use std::fs::File;
use std::io::{self, BufRead, Read, Error};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_fully(filename: &str) -> Vec<String> {
    let mut the_lines = Vec::<String>::new();

    let line_read_result = read_lines(filename);

    if let Ok(lines) = line_read_result {
        for line in lines {
            if let Ok(the_line) = line {
                the_lines.push(the_line);
            }
        }
    }

    return the_lines;
}
