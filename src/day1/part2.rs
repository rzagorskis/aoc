use super::utils::char_digit_to_digit;
use super::utils::read_lines_from_input_file;

/*

--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?


*/


fn extract_calibation_value_from_line(line: &String) -> usize {
  return 0;
}

pub fn run() {
  let mut calibration_sum = 0;

  let lines_result = read_lines_from_input_file();

  if let Ok(lines) = lines_result {
      for line in lines {
          if let Ok(ip) = line {
              calibration_sum += extract_calibation_value_from_line(&ip);
          }
      }
  }

  println!(
      "Part 2 -> Total calibration sum of all string lines: {}",
      calibration_sum
  );
}
