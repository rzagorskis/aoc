/*
--- Part Two ---

The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

For example:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

    Step 0: You are at 11A and 22A.
    Step 1: You choose all of the left paths, leading you to 11B and 22B.
    Step 2: You choose all of the right paths, leading you to 11Z and 22C.
    Step 3: You choose all of the left paths, leading you to 11B and 22Z.
    Step 4: You choose all of the right paths, leading you to 11Z and 22B.
    Step 5: You choose all of the left paths, leading you to 11B and 22C.
    Step 6: You choose all of the right paths, leading you to 11Z and 22Z.

So, in this example, you end up entirely on nodes that end in Z after 6 steps.

Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?

*/

use std::collections::HashMap;

use crate::{io_utils::read_lines_fully, math::calculate_lcm};

type Direction = String;
type LeftRightDirections = (String, String);
type StepsTaken = u64;
type Walking = bool;

const EXPECTED_ANSWER: u64 = 13129439557681;

fn parse() -> (
    String,
    Vec<(Direction, LeftRightDirections)>,
    HashMap<Direction, LeftRightDirections>,
) {
    let lines = read_lines_fully("src/day8/input.txt")
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut iterator = lines.iter();

    let directions = iterator.next().unwrap();

    let mut directional_map = HashMap::<Direction, LeftRightDirections>::new();

    let mut starting_points = Vec::<(Direction, LeftRightDirections)>::new();

    for mapping_line in iterator {
        let split = mapping_line.split(" = ").collect::<Vec<_>>();

        let direction_key = split.first().unwrap();
        let key_directions = split
            .last()
            .unwrap()
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|item| item.trim())
            .collect::<Vec<_>>();

        if direction_key.ends_with('A') {
            starting_points.push((
                direction_key.to_string(),
                (
                    key_directions.first().unwrap().to_string(),
                    key_directions.last().unwrap().to_string(),
                ),
            ));
        }

        directional_map.insert(
            direction_key.to_string(),
            (
                key_directions.first().unwrap().to_string(),
                key_directions.last().unwrap().to_string(),
            ),
        );
    }

    return (
        directions.to_string(),
        starting_points,
        directional_map,
    );
}

pub fn run() {
    let directions = parse();

    let mut current_stops_for_walk_id = HashMap::<usize, (Direction, LeftRightDirections)>::new();
    let mut walk_id_state = HashMap::<usize, (StepsTaken, Walking)>::new();

    directions
        .1
        .iter()
        .enumerate()
        .for_each(|(id, starting_point)| {
            current_stops_for_walk_id.insert(id, starting_point.clone());
            walk_id_state.insert(id, (0, true));
        });

    println!("Directions: {}", directions.0);
    println!();
    println!("All starting points: {:?}", current_stops_for_walk_id);
    println!();

    let walk_ids = current_stops_for_walk_id
        .iter()
        .map(|(walk_id, _)| walk_id.clone())
        .collect::<Vec<_>>();

    for walk_id in walk_ids {
        loop {
            for direction in directions.0.chars() {
                let current_stop_point = current_stops_for_walk_id.get(&walk_id).unwrap();
    
                // println!("walk_id: {}, {:?}", walk_id, current_stop_point);
    
                if current_stop_point.0.ends_with('Z') {
                    println!(
                        "Found stop ending with Z for walk id: {}, with steps taken of: {}",
                        walk_id,
                        walk_id_state.get(&walk_id).unwrap().0
                    );
                    println!();

                    walk_id_state.get_mut(&walk_id).unwrap().1 = false;
    
                    break;
                }
    
                if let Some(walk_id_step) = walk_id_state.get_mut(&walk_id) {
                    walk_id_step.0 += 1;
                }
    
                if direction == 'L' {
                    current_stops_for_walk_id.insert(
                        walk_id,
                        (
                            current_stop_point.1 .0.clone(),
                            directions.2.get(&current_stop_point.1 .0).unwrap().clone(),
                        ),
                    );
                } else {
                    current_stops_for_walk_id.insert(
                        walk_id,
                        (
                            current_stop_point.1 .1.clone(),
                            directions.2.get(&current_stop_point.1 .1).unwrap().clone(),
                        ),
                    );
                }
            }

            let walking = walk_id_state.get_mut(&walk_id).unwrap().1;

            if !walking {
                break;
            }
        }
    }

    let lcm_steps = calculate_lcm(walk_id_state.iter().map(|item| item.1.0).collect::<Vec<_>>().as_slice());

    assert_eq!(EXPECTED_ANSWER, lcm_steps);

    println!("LCM Steps to take: {}",  lcm_steps);
}
