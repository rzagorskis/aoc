use crate::io_utils::{read_lines_fully, extract_numbers_from_line};


#[derive(PartialEq)]
pub enum NumberPredictionType {
    Previous,
    Next
}

pub fn parse() -> Vec<Vec<i32>> {
    let number_lines = read_lines_fully("src/day9/input.txt")
        .iter()
        .map(|line| extract_numbers_from_line::<i32>(line))
        .collect::<Vec<_>>();

    number_lines
}

pub fn build_history_sequence(number_history: &mut Vec<Vec<i32>>) {
    let last = number_history.last().unwrap();
    if last.iter().all(|value| value == &0) {
        return;
    }

    let mut diff_at_depth = Vec::<i32>::new();

    let iterations = last.len() - 1;

    for ix in 0..iterations {
        let first = last.iter().nth(ix).unwrap();
        let second = last.iter().nth(ix + 1).unwrap();

        diff_at_depth.push(second - first);
    }

    number_history.push(diff_at_depth);

    return build_history_sequence(number_history);
}

pub fn predict_next_number(number_history: &Vec<Vec<i32>>, side: NumberPredictionType) -> i32 {
    let mut next_value = 0;

    for seq in number_history.iter().rev().skip(1) {
        // println!("{:?}", seq);
        if side == NumberPredictionType::Next {
            next_value += seq.last().unwrap() 
        } else {
            next_value = seq.first().unwrap() - next_value;
        }
    }

    return next_value;
}