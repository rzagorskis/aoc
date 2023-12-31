use std::path::Path;

use crate::io_utils::read_lines;
use super::utils::vec_char_to_calibration_value;

/*

--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

const EXPECTED_ANSWER: usize = 54968;

fn extract_calibation_value_from_line(line: &String) -> usize {
    let mut calibration_chars = Vec::<char>::new();

    // TODO: would be nicer to ony build the chars vec up to max of 2 in size
    for char in line.chars() {
        if char.is_numeric() {
            calibration_chars.push(char);
        }
    }

    return vec_char_to_calibration_value(&calibration_chars);
}

pub fn run() {
    let mut calibration_sum = 0;

    let lines_result = read_lines(Path::new("src/day1/input.txt"));

    if let Ok(lines) = lines_result {
        for line in lines {
            if let Ok(ip) = line {
                calibration_sum += extract_calibation_value_from_line(&ip);
            }
        }
    }

    assert_eq!(
        calibration_sum, EXPECTED_ANSWER,
        "expected {}, got {}",
        EXPECTED_ANSWER, calibration_sum
    );

    println!(
        "Part 1 -> Total calibration sum of all string lines: {}",
        calibration_sum
    );
}
