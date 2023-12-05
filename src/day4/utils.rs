use std::collections::{HashSet, HashMap};

use crate::io_utils::read_lines;

use super::defs::CardState;

pub fn read_card_lines() -> Vec<String> {
    let mut card_lines = Vec::<String>::new();

    let line_read_result = read_lines("src/day4/input.txt");

    if let Ok(lines) = line_read_result {
        for line in lines {
            if let Ok(the_line) = line {
                card_lines.push(the_line);
            }
        }
    }

    return card_lines;
}

pub fn parse_card_lines(card_lines: Vec<String>) -> HashMap<usize, CardState> {
    let mut card_maps = HashMap::<usize, CardState>::new();

    for (ix, card_line) in card_lines.iter().enumerate() {
        let first_split: Vec<&str> = card_line.split(":").collect();

        let numbers_split: Vec<&str> = first_split.get(1).unwrap().split(" | ").collect();

        let winning_numbers: Result<HashSet<usize>, _> = numbers_split
            .get(0)
            .unwrap()
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.parse())
            .collect();

        let my_numbers: Result<HashSet<usize>, _> = numbers_split
            .get(1)
            .unwrap()
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.parse())
            .collect();

        if let (Ok(winning_numbers), Ok(my_numbers)) = (winning_numbers, my_numbers) {
            card_maps.insert(ix, CardState { winning_numbers, my_numbers });
        }
    }

    return card_maps;
}
