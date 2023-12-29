/*

--- Day 2: Cube Conundrum ---

You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

*/

use crate::day2::utils::parse_games_from_lines;

use super::defs::Game;

mod game_eval_module {
    use super::Game;

    const MAX_CUBES_RED: usize = 12;
    const MAX_CUBES_GREEN: usize = 13;
    const MAX_CUBES_BLUE: usize = 14;

    #[derive(Debug)]
    pub struct GameEval<'a> {
        game: &'a Game,
        evaluated: bool,
        is_possible: bool,
    }

    impl<'a> GameEval<'a> {
        pub fn new(game: &'a Game) -> Self {
            GameEval {
                game,
                evaluated: false,
                is_possible: true,
            }
        }

        pub fn evaluate(&mut self) {
            if self.evaluated {
                return;
            }

            for game_set in &self.game.sets {
                if let Some(red) = game_set.red {
                    if red > MAX_CUBES_RED {
                        self.is_possible = false;
                        break;
                    }
                }

                if let Some(green) = game_set.green {
                    if green > MAX_CUBES_GREEN {
                        self.is_possible = false;
                        break;
                    }
                }

                if let Some(blue) = game_set.blue {
                    if blue > MAX_CUBES_BLUE {
                        self.is_possible = false;
                        break;
                    }
                }
            }

            self.evaluated = true;
        }

        pub fn is_evaluated(&self) -> bool {
            return self.evaluated;
        }

        pub fn is_possible(&self) -> bool {
            return self.is_possible;
        }

        pub fn game(&self) -> &Game {
            return self.game;
        }

        pub fn print_debug(&self) {
            println!("{} -> is_possible: {}", self.game.ref_line, self.is_possible);
        }
    }

    #[derive(Debug)]
    pub struct GameEvalResult<'a> {
        pub game_evals: Vec<GameEval<'a>>,
        pub total_possible: usize,
        pub sum_of_game_ids: usize
    }
}

fn eval_games(games: &Vec<Game>) -> game_eval_module::GameEvalResult {
    let mut game_evals = Vec::<game_eval_module::GameEval>::with_capacity(games.len());
    let mut total_possible = 0;
    let mut sum_of_game_ids = 0;

    for game in games {
        let mut eval = game_eval_module::GameEval::new(game);

        eval.evaluate();

        if eval.is_possible() {
            total_possible += 1;
            sum_of_game_ids += eval.game().id;
        }

        game_evals.push(eval);
    }

    return game_eval_module::GameEvalResult {
        game_evals,
        total_possible,
        sum_of_game_ids
    };
}

const EXPECTED_ANSWER: usize = 2563;

pub fn run() {
    let games = parse_games_from_lines();
    let eval_result = eval_games(&games);

    assert_eq!(EXPECTED_ANSWER, eval_result.sum_of_game_ids, "Expected {}, got {}", EXPECTED_ANSWER, eval_result.sum_of_game_ids);

    println!("Sum of game ids: {:?}", eval_result.sum_of_game_ids);
}
