use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_from_input_file() -> io::Result<Lines<BufReader<File>>> {
  let this_file_path = Path::new(file!());

  let input_file_relative = &format!(
      "{}{}",
      this_file_path.parent().unwrap().display(),
      "/input.txt"
  );
  let input_file_path = Path::new(input_file_relative.as_str());

  println!("{}", input_file_path.display());

  return read_lines(input_file_path);
}

pub fn char_digit_to_digit(char_digit: &char) -> usize {
  return char_digit.to_digit(10).unwrap() as usize;
}
