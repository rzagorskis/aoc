use phf::phf_map;

use super::utils::read_lines_from_input_file;
use super::utils::vec_char_to_calibration_value;

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

static WORD_TO_NUMBER_MAP: phf::Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

const EXPECTED_ANSWER: usize = 54676;

fn extract_calibation_value_from_line(line: &String) -> usize {
    let mut calibration_chars = Vec::<char>::new();
    let mut word_char_str = String::from("");

    for char in line.chars() {
        if char.is_numeric() {
            word_char_str.clear();
            calibration_chars.push(char);
            continue;
        }

        word_char_str.push(char);

        for (ix, word_char) in word_char_str.char_indices() {
            let key = &word_char_str[ix..word_char_str.len()];
            let optional_num_value = WORD_TO_NUMBER_MAP.get(key).cloned();

            if let Some(num_value) = optional_num_value {
                calibration_chars.push(num_value);

                // cover cases like 'oneight' where it should produce 1 and 8
                let last_ix = word_char_str.len() - 1;
                let last_char = word_char_str[last_ix..=last_ix].to_string();

                word_char_str.clear();
                word_char_str.push(last_char.chars().nth(0).unwrap());

                break;
            }
        }
    }

    let calibration_value = vec_char_to_calibration_value(&calibration_chars);

    println!("{} -> {:?} -> {}", line, &calibration_chars, &calibration_value);

    return calibration_value;
}

pub fn run() {
    let mut calibration_sum = 0;

    // let lines_result = read_lines_from_input_file(Option::Some("/input_simple.txt"));
    let lines_result = read_lines_from_input_file(Option::None);

    if let Ok(lines) = lines_result {
        for line in lines {
            if let Ok(ip) = line {
                calibration_sum += extract_calibation_value_from_line(&ip);
            }
        }
    }

    assert_eq!(calibration_sum, EXPECTED_ANSWER.to_owned(), "expected {}, got {}", EXPECTED_ANSWER.to_owned(), calibration_sum);

    println!(
        "Part 2 -> Total calibration sum of all string lines: {}",
        calibration_sum
    );
}
