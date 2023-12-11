/*
--- Day 8: Haunted Wasteland ---

You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)

Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?


*/

use std::collections::HashMap;

use crate::io_utils::read_lines_fully;

const EXPECTED_ANSWER: u32 = 13771;

fn parse() -> (String, (String, (String, String)), HashMap<String, (String, String)>) {
    let lines = read_lines_fully("src/day8/input.txt")
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut iterator = lines.iter();

    let directions = iterator.next().unwrap();

    println!("{}", directions);

    let mut directional_map = HashMap::<String, (String, String)>::new();

    let mut starting_point = None::<(String, (String, String))>;

    for (ix, mapping_line) in iterator.enumerate() {
        let split = mapping_line.split(" = ").collect::<Vec<_>>();

        let direction_key = split.first().unwrap();
        let key_directions = split
            .last()
            .unwrap()
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|item| item.trim())
            .collect::<Vec<_>>();

        if direction_key == &"AAA" {
            starting_point = Some(
                (
                    direction_key.to_string(), 
                    (
                        key_directions.first().unwrap().to_string(),
                        key_directions.last().unwrap().to_string(),
                    )
                )
            )
        }

        directional_map.insert(
            direction_key.to_string(),
            (
                key_directions.first().unwrap().to_string(),
                key_directions.last().unwrap().to_string(),
            ),
        );
    }

    return (directions.to_string(), starting_point.unwrap(), directional_map);
}

pub fn run() {
    let directions = parse();

    let mut walking = true;
    let mut steps_taken = 0;
    let mut current_stop = directions.1; 

    let mut full_pattern_iteration_attempts = 0;

    while walking {
        for direction in directions.0.chars() { 
            steps_taken += 1;

            // println!("Curr stop: {:?}", current_stop);

            if direction == 'L' {
                if current_stop.1.0 == "ZZZ" {
                    walking = false;
                    break;
                }

                current_stop = (current_stop.1.0.clone(), directions.2.get(&current_stop.1.0).unwrap().clone());
            } else {
                if current_stop.1.1 == "ZZZ" {
                    walking = false;
                    break;
                }

                // right
                current_stop = (current_stop.1.1.clone(), directions.2.get(&current_stop.1.1).unwrap().clone());
            }
        }


        full_pattern_iteration_attempts += 1;

        

        println!("Walked entire pattern with no exit for iteration: {}", full_pattern_iteration_attempts);
    }

    assert_eq!(EXPECTED_ANSWER, steps_taken);

    println!("Steps taken for: {} -> {}", directions.0, steps_taken);
}
