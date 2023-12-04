/*
--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

*/

use crate::io_utils::read_lines;

#[derive(Debug, Clone)]
struct NumericChar {
    ix: usize,
    char: char,
}

#[derive(Debug)]
struct LineMeta {
    num_chars: Vec<NumericChar>,
    ix: usize,
    len: usize,
}

const BLANK_SYMBOL: char = '.';

fn read_schematic() -> Vec<String> {
    let mut schematic = Vec::<String>::new();

    let lines_result = read_lines("src/day3/input_sample.txt");

    if let Ok(lines) = lines_result {
        for line in lines {
            if let Ok(the_line) = line {
                schematic.push(the_line);
            }
        }
    }

    return schematic;
}

fn num_buffer_to_number(buffer: &Vec<NumericChar>) -> usize {
    let mut number_str = String::with_capacity(buffer.len());

    for meta in buffer {
        number_str.push(meta.char);
    }

    return number_str.parse().unwrap();
}

fn extract_line_meta(schematic: &Vec<String>) -> Vec<LineMeta> {
    let mut all_numbers = Vec::<LineMeta>::new();

    for (line_ix, line) in schematic.iter().enumerate() {
        println!("{}", line);

        let mut num_buffer = Vec::<NumericChar>::new();

        for (line_char_ix, char) in line.char_indices() {
            if char.is_numeric() {
                num_buffer.push(NumericChar {
                    ix: line_char_ix,
                    char,
                });

                if line_char_ix == line.len() - 1 {
                    all_numbers.push(LineMeta {
                        ix: line_ix,
                        num_chars: num_buffer.clone(),
                        len: line.len(),
                    });
                    num_buffer.clear();
                }
            } else {
                if num_buffer.len() > 0 {
                    all_numbers.push(LineMeta {
                        ix: line_ix,
                        num_chars: num_buffer.clone(),
                        len: line.len(),
                    });
                    num_buffer.clear();
                }
            }
        }
    }

    return all_numbers;
}

fn is_symbol_char(the_char: &char) -> bool {
    return *the_char != BLANK_SYMBOL && !the_char.is_numeric();
}

fn is_symbol_around(
    ix: usize,
    number: &NumericChar,
    line_meta: &LineMeta,
    line_current: &[u8],
    line_below: Option<&[u8]>,
    line_above: Option<&[u8]>,
) -> bool {
    if number.ix == 0 {
        return false;
    }

    if number.ix == line_meta.len - 1 {
        return false;
    }

    if ix == 0 {
        let char_at_left = *line_current.get(number.ix - 1).unwrap() as char;

        if is_symbol_char(&char_at_left) {
            return true;
        }

        if let Some(line_below) = line_below {
            let char_at_left_below = *line_below.get(number.ix - 1).unwrap() as char;

            if is_symbol_char(&char_at_left_below) {
                return true;
            }
        }

        if let Some(line_above) = line_above {
            let char_at_left_above = *line_above.get(number.ix - 1).unwrap() as char;

            if is_symbol_char(&char_at_left_above) {
                return true;
            }
        }
    } else if ix == line_meta.num_chars.len() - 1 {
        let char_at_right = *line_current.get(number.ix + 1).unwrap() as char;

        if is_symbol_char(&char_at_right) {
            return true;
        }

        if let Some(line_below) = line_below {
            let char_at_right_below = *line_below.get(number.ix + 1).unwrap() as char;

            if is_symbol_char(&char_at_right_below) {
                return true;
            }
        }

        if let Some(line_above) = line_above {
            let char_at_right_above = *line_above.get(number.ix + 1).unwrap() as char;

            if is_symbol_char(&char_at_right_above) {
                return true;
            }
        }
    }

    return false;
}

fn calc_valid_parts_sum(valid_parts: Vec<Vec<NumericChar>>) -> usize {
    let mut total_sum = 0;

    for valid_part in valid_parts {
        
        
        let part_num = num_buffer_to_number(&valid_part);

        println!("{}", part_num);

        total_sum += part_num;
    }

    return total_sum;
}

fn analyse_schematic(schematic: Vec<String>) {
    let line_metas = extract_line_meta(&schematic);

    let mut valid_parts = Vec::<(usize, Vec<NumericChar>)>::new();

    for line_meta in line_metas {
        // println!("checking -> {:?}", line_meta.num_chars);
        let mut is_valid = false;

        let this_line = schematic.get(line_meta.ix).unwrap().as_bytes();

        for (ix, number) in line_meta.num_chars.iter().enumerate() {
            if line_meta.ix == 0 {
                let line_below_chars = schematic.get(line_meta.ix + 1).unwrap().as_bytes();

                if is_symbol_around(ix, number, &line_meta, &this_line, Some(&line_below_chars), None) {
                    is_valid = true;
                    break;
                }

                let char_below = *line_below_chars.get(number.ix).unwrap() as char;
                if is_symbol_char(&char_below) {
                    is_valid = true;
                    break;
                }
            } else if line_meta.ix == schematic.len() - 1 {
                let line_above_chars = schematic.get(line_meta.ix - 1).unwrap().as_bytes();

                if is_symbol_around(ix, number, &line_meta, &this_line, None, Some(&line_above_chars)) {
                    is_valid = true;
                    break;
                }

                let char_above = *line_above_chars.get(number.ix).unwrap() as char;
                if is_symbol_char(&char_above) {
                    is_valid = true;
                    break;
                }
            } else {
                let line_above_chars = schematic.get(line_meta.ix - 1).unwrap().as_bytes();
                let line_below_chars = schematic.get(line_meta.ix + 1).unwrap().as_bytes();

                if is_symbol_around(
                    ix,
                    number,
                    &line_meta,
                    &this_line,
                    Some(&line_below_chars),
                    Some(&line_above_chars),
                ) {
                    is_valid = true;
                    break;
                }

                let char_above = *line_above_chars.get(number.ix).unwrap() as char;
                if is_symbol_char(&char_above) {
                    is_valid = true;
                    break;
                }

                let char_below = *line_below_chars.get(number.ix).unwrap() as char;
                if is_symbol_char(&char_below) {
                    is_valid = true;
                    break;
                }
            }
        }

        if is_valid {
            valid_parts.push((line_meta.ix, line_meta.num_chars));
        }
    }

    let total_sum = calc_valid_parts_sum(valid_parts.iter().map(|part| part.1.clone()).collect());

    println!("Total sum of all valid parts: {}", total_sum);
}

pub fn run() {
    analyse_schematic(read_schematic());
}
