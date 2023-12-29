/*

--- Part Two ---

The galaxies are much older (and thus much farther apart) than the researcher initially estimated.

Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

(In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?


*/

const EXPECTED_ANSWER: i64 = 363293506944;

use std::cmp::{max, min};

use crate::math::manhattan_distance;

const INPUT: &'static str = include_str!("input.txt");

const IMAGE_CHAR_EMPTY_SPACE: char = '.';
const IMAGE_CHAR_GALAXY: char = '#';
const EMPTY_ROW_OR_COL_FACTOR: i64 = 999999;

fn count_intersecting_indices(indices: &Vec<usize>, indice_start_end: (usize, usize)) -> i64 {
    let matching_indices = indices
        .iter()
        .filter(|indice| {
            let min_point = min(indice_start_end.0, indice_start_end.1);
            let max_point = max(indice_start_end.0, indice_start_end.1);
            min_point <= **indice && **indice <= max_point
        })
        .count() as i64;

    return matching_indices;
}

fn expand(galaxy: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_row_indices: Vec<usize> = Vec::new();
    let mut empty_col_indices: Vec<usize> = Vec::new();

    galaxy.iter().enumerate().for_each(|(i, v)| {
        if !v.contains(&IMAGE_CHAR_GALAXY) {
            empty_row_indices.push(i);
        }
    });

    for i in 0..galaxy[0].len() {
        let found = galaxy.iter().any(|row| row[i] == IMAGE_CHAR_GALAXY);
        if !found {
            empty_col_indices.push(i);
        }
    }

    (empty_row_indices, empty_col_indices)
}

pub fn run() {
    let input = INPUT
        .lines()
        .map(|line| {
            let mut chars = Vec::<char>::new();
            for char in line.chars() {
                chars.push(char);
            }

            chars
        })
        .collect::<Vec<_>>();

    let mut indices: Vec<(usize, usize)> = Vec::new();
    let mut total_sum = 0;

    input.iter().enumerate().for_each(|(i, v)| {
        v.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                indices.push((i, j));
            }
        })
    });

    //println!("{:?}", indices);
    
    let (expanded_rows_indices, expanded_cols_indices) = expand(&input);

    println!("Rows: {:?}", expanded_rows_indices);
    println!("Cols: {:?}", expanded_cols_indices);

    for (i, start_indice) in indices.iter().enumerate() {
        for j in i + 1..indices.len() {
            let goal_indice = indices[j];

            let manhattan = manhattan_distance(
                start_indice.0 as i64,
                start_indice.1 as i64,
                goal_indice.0 as i64,
                goal_indice.1 as i64,
            );

            let row_expansion_factor = count_intersecting_indices(&expanded_rows_indices, (start_indice.0, goal_indice.0));
            let rows_factor = row_expansion_factor * EMPTY_ROW_OR_COL_FACTOR;

            let col_expansion_factor = count_intersecting_indices(&expanded_cols_indices, (start_indice.1, goal_indice.1));
            let cols_factor = col_expansion_factor * EMPTY_ROW_OR_COL_FACTOR;

            // println!("start: {:?}, goal: {:?}, row_factor: {}, col_factor: {}", start_indice, goal_indice, row_expansion_factor, col_expansion_factor);

            total_sum += manhattan + rows_factor + cols_factor
        }
    }

    assert_eq!(EXPECTED_ANSWER, total_sum);

    println!("Result: {}", total_sum);
}

