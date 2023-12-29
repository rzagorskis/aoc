use crate::io_utils::read_lines;

use super::defs::{Game, GameSet};

pub fn parse_game_line(line: &str) -> Game {
    let initial_split: Vec<&str> = line.split(":").collect();

    assert_eq!(initial_split.len(), 2);

    let game_meta_split: Vec<&str> = initial_split.get(0).unwrap().split(" ").collect();
    let game_id: usize = game_meta_split.last().unwrap().parse().unwrap();
    
    let set_splits: Vec<&str> = initial_split.get(1).unwrap().split(";").collect();
    let mut game_sets: Vec<GameSet> = Vec::with_capacity(set_splits.len());

    for set in set_splits {
        let collected_cubes: Vec<&str> = set.trim().split(", ").collect();

        let mut red = Option::<usize>::None;
        let mut green = Option::<usize>::None;
        let mut blue = Option::<usize>::None;
        
        for collected_cube in collected_cubes {
            let cube_meta: Vec<&str> = collected_cube.split(" ").collect();

            let amount: usize = cube_meta.get(0).unwrap().parse().unwrap();
            let cube_color = cube_meta.get(1).unwrap().to_owned();

            match cube_color {
                "red" => red = Option::Some(amount),
                "green" => green = Option::Some(amount),
                "blue" => blue = Option::Some(amount),
                _other => {}
            }
        }

        game_sets.push(GameSet { red, green, blue })
    }
    
    return Game {
        id: game_id,
        sets: game_sets,
        ref_line: line.to_owned(),
    }
}

pub fn parse_games_from_lines() -> Vec<Game> {
    let lines_result = read_lines("src/day2/input.txt");
    let mut parsed_games = Vec::<Game>::new();

    if let Ok(lines) = lines_result {
        for line in lines {
            if let Ok(the_line) = line {
                parsed_games.push(parse_game_line(&the_line))
            }
        }
    }

    return parsed_games;
}
