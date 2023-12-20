/*
--- Day 11: Cosmic Expansion ---

You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....

The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^

These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......

Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......

In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......

This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:

    Between galaxy 1 and galaxy 7: 15
    Between galaxy 3 and galaxy 6: 17
    Between galaxy 8 and galaxy 9: 5

In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

*/

const IMAGE_CHAR_EMPTY_SPACE: char = '.';
const IMAGE_CHAR_GALAXY: char = '#';

const INPUT: &'static str = include_str!("input.txt");
const SAMPLE: &'static str = include_str!("input_sample.txt");

use std::collections::HashSet;
use std::str::Lines;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;

use aoc_2023::Point;
use pathfinding::directed::astar::astar;

use crate::path_finding::{find_all_unique_start_goal_points, StartGoal};

fn expand_galaxy(lines: Lines) -> Vec<Vec<char>> {
    let mut expanded_galaxy_lines = Vec::<Vec<char>>::new();

    for line in lines {
        // println!("{}", line);
        let mut chars = Vec::<char>::with_capacity(line.len());
        for char in line.chars() {
            chars.push(char);
        }

        expanded_galaxy_lines.push(chars);
    }


    { // expand rows
        let mut current_row_ix = 0;
        let mut iterated = 0;
        let iterations = expanded_galaxy_lines.len() - 1;

        
        while iterated < iterations {
            let row = expanded_galaxy_lines.get(current_row_ix).unwrap();

            if row.iter().all(|char| *char == IMAGE_CHAR_EMPTY_SPACE) {
                expanded_galaxy_lines.insert(current_row_ix, row.clone());
                current_row_ix += 2;
            } else {
                current_row_ix += 1;
            }
            
            iterated += 1;
        }
    }
    

    { // expand cols
        let mut current_col_ix = 0;
        let mut iterated_cols = 0;
        let max_col_iterations = expanded_galaxy_lines.first().unwrap().len() - 1;

        while iterated_cols < max_col_iterations {

            let mut have_galaxy_on_col = false;

            for row in expanded_galaxy_lines.iter() {
                let char_at_row_and_col = row.get(current_col_ix).unwrap();

                if *char_at_row_and_col == IMAGE_CHAR_GALAXY {
                    have_galaxy_on_col = true;
                    break;
                }
            }

            if have_galaxy_on_col {
                current_col_ix += 1;
            } else {

                for row in 0..expanded_galaxy_lines.len() {
                    let chars = expanded_galaxy_lines.get_mut(row).unwrap();

                    chars.insert(current_col_ix, IMAGE_CHAR_EMPTY_SPACE);
                }

                current_col_ix += 2;
            }

            iterated_cols += 1;
        }
    }

    return expanded_galaxy_lines;
}

fn find_galaxy_locations(galaxy: &Vec<Vec<char>>) -> Vec::<Point> {
    let mut points = Vec::<Point>::new();

    for (row_ix, row) in galaxy.iter().enumerate() {
        for (col_ix, char) in row.iter().enumerate() {
            if *char == IMAGE_CHAR_GALAXY {
                points.push(Point{ x: row_ix as i32, y: col_ix as i32 });
            }
        }
    }

    return points;
}

fn async_calc_chunks(id: usize, pairs: Vec<StartGoal>) -> u32 {
    let mut chunk_total = 0;

    for start_goal in pairs {
        let result = astar(
            &(start_goal.start.x, start_goal.start.y),
            |&(x, y)| {
                let successor = vec![
                    (x + 1, y),
                    (x - 1, y),
                    (x, y + 1),
                    (x, y - 1),
                ]
                .into_iter()
                .map(|p| (p, 1));
    
                successor
            },
            |&(x, y)| (start_goal.goal.x.abs_diff(x) + start_goal.goal.y.abs_diff(y)) / 3,
            |&p| p == (start_goal.goal.x, start_goal.goal.y),
        );

        if let Some(result) = result {
            // if ix == 0 ||  ix % 250 == 0 {
            //     println!("ID ({}) => ix {}: {:?} -> {:?} = {:?}", id, ix, start_goal.start, start_goal.goal, result.1);
            // }
            chunk_total += result.1;
        } else {
            println!("ID ({}) => Issue calculating path for {:?} -> {:?} = {:?}", id, start_goal.start, start_goal.goal, result);
        }
    }

    return chunk_total;
}

fn build_processing_chunks(all: &HashSet<StartGoal>) -> Vec::<Vec<StartGoal>> {
    const CHUNK_SIZE: usize = 500;

    let mut chunks = Vec::<Vec<StartGoal>>::new();
    let mut temp = Vec::<StartGoal>::with_capacity(CHUNK_SIZE as usize);

    for (ix, entry) in all.iter().enumerate() {
        temp.push(*entry);

        if temp.len() == CHUNK_SIZE || ix == all.len() - 1 {
            chunks.push(temp.clone());
            temp.clear();
        }
    }

    return chunks;
}

fn calc_chunks(chunks: Vec::<Vec<StartGoal>>, total_to_process: usize) -> u32 {
    let remaining = Arc::new(AtomicUsize::new(total_to_process));

    println!("Total remaining: {}", remaining.load(Ordering::SeqCst));

    return chunks
        .par_iter()
        .enumerate()
        .map(|item| {
            let total_in_chunk = item.1.len();
            let result = async_calc_chunks(item.0, item.1.to_vec());

            remaining.fetch_sub(total_in_chunk, Ordering::SeqCst);

            println!("Total remaining: {}", remaining.load(Ordering::SeqCst));

            result
        })
        .sum::<u32>();
}

const EXPECTED_ANSWER: usize = 9_233_514;

pub fn run() {
    let expanded = expand_galaxy(INPUT.lines());
    let points = find_galaxy_locations(&expanded);

    let all_start_goal_pairs = find_all_unique_start_goal_points(&points);

    let chunks = build_processing_chunks(&all_start_goal_pairs);

    let total_sum = calc_chunks(chunks, all_start_goal_pairs.len());

    assert_eq!(EXPECTED_ANSWER, total_sum as usize);

    println!("Total sum: {}", total_sum);
}
