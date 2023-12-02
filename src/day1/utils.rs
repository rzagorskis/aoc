use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_from_input_file(file_name: Option<&str>) -> io::Result<Lines<BufReader<File>>> {
    let this_file_path = Path::new(file!());

    let input_file_relative = &format!(
        "{}{}",
        this_file_path.parent().unwrap().display(),
        file_name.unwrap_or("/input.txt")
    );
    let input_file_path = Path::new(input_file_relative);

    return read_lines(input_file_path);
}

pub fn char_digit_to_digit(char_digit: &char) -> usize {
    return char_digit.to_digit(10).unwrap() as usize;
}

pub fn digit_parts_to_two_digit_number(digit_one: usize, digit_two: Option<usize>) -> usize {
    return String::from(format!(
        "{}{}",
        digit_one,
        digit_two.or(Some(digit_one)).unwrap()
    ))
    .parse()
    .unwrap();
}

pub fn chars_to_two_digit_number(digit_one: &char, digit_two: Option<&char>) -> usize {
    return digit_parts_to_two_digit_number(
        char_digit_to_digit(digit_one),
        Some(char_digit_to_digit(digit_two.unwrap())),
    );
}

pub fn vec_char_to_calibration_value(calibration_chars: &Vec<char>) -> usize {
  if calibration_chars.len() == 0 {
      return 0;
  }

  if calibration_chars.len() == 1 {
      let the_digit = calibration_chars.get(0).unwrap();

      return chars_to_two_digit_number(the_digit, Some(the_digit));
  }

  if calibration_chars.len() == 2 {
      return chars_to_two_digit_number(
          calibration_chars.get(0).unwrap(),
          Some(calibration_chars.get(1).unwrap()),
      );
  }

  return chars_to_two_digit_number(
      calibration_chars.get(0).unwrap(),
      Some(calibration_chars.get(calibration_chars.len() - 1).unwrap()),
  );
}