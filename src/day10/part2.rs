/*
--- Part Two ---

You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........

The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....

In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........

In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...

The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO

In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?

*/


use std::collections::HashSet;

use crate::day10::utils::{
    build, get_grid_item, Direction, FromDirection, MoveDirectionCombo, Tile,
};

use super::utils::PotentialGridLocation;

const EXPECTED_ANSWER: i32 = -1;

#[derive(Debug, Clone, Copy)]
struct LocationMeta {
    came_from: FromDirection,
    location: PotentialGridLocation,
    tile: Tile,
}

#[derive(Debug)]
enum LoopDirection {
    Clockwise,
    Counterclockwise,
    Unknown
}

fn overall_orientation(points: &Vec::<LocationMeta>) -> LoopDirection {
    if points.len() < 3 {
        panic!("Insufficient points for orientation determination");
    }

    let mut overall_det = 0;

    for i in 0..points.len() {
        let next_i = (i + 1) % points.len();
        overall_det += (points[next_i].location.0 - points[i].location.0) * (points[next_i].location.1 + points[i].location.1);
    }

    if overall_det < 0 {
        LoopDirection::Counterclockwise
    } else if overall_det > 0 {
        LoopDirection::Clockwise
    } else {
        LoopDirection::Unknown
    }
}

fn determine_loop_direction(loop_points: &Vec::<LocationMeta>) -> LoopDirection{
    let mut cross_product_sum = 0.0;

    for i in 0..loop_points.len() {
        let next_i = (i + 1) % loop_points.len();
        let current_point = &loop_points[i];
        let next_point = &loop_points[next_i];

        cross_product_sum += (next_point.location.0 as f64 - current_point.location.1 as f64) * (next_point.location.0 as f64 + current_point.location.1 as f64);
    }

    if cross_product_sum > 0.0 {
        LoopDirection::Clockwise
    } else if cross_product_sum < 0.0 {
        LoopDirection::Counterclockwise
    } else {
        LoopDirection::Unknown
    }
}

pub fn run() {
    let state = build();

    // figure out the loop direction we can go first
    // then once we have a direction, we can start the move loop
    let starting_point = state.0;
    let points: [(FromDirection, (isize, isize)); 4] = [
        (
            FromDirection::South,
            (starting_point.0 as isize - 1, starting_point.1 as isize),
        ),
        (
            FromDirection::North,
            (starting_point.0 as isize + 1, starting_point.1 as isize),
        ),
        (
            FromDirection::West,
            (starting_point.0 as isize, starting_point.1 as isize + 1),
        ),
        (
            FromDirection::East,
            (starting_point.0 as isize, starting_point.1 as isize - 1),
        ),
    ];

    let current_point = points
        .iter()
        .find(|next_point| {
            let item = get_grid_item(&next_point.1, &state);

            if let Some(item) = item {
                // direction you're coming + the tile you're on now
                let next_moves = state
                    .2
                    .get(&MoveDirectionCombo(next_point.0.clone(), item.clone()));

                if let Some(next_moves) = next_moves {
                    if next_moves.contains(item) {
                        return true;
                    }
                }
            }

            return false;
        })
        .unwrap();

    let mut looping = true;
    let mut direction_came_from = current_point.0.clone();
    let mut current_point_temp = current_point.1.clone();

    let mut loop_chain = Vec::<LocationMeta>::new();

    while looping {
        let current_points_tile = get_grid_item(&current_point_temp, &state).unwrap();

        loop_chain.push(LocationMeta {
            came_from: direction_came_from,
            location: current_point_temp.clone(),
            tile: *current_points_tile,
        });

        if *current_points_tile == Tile::StartingPoint {
            looping = false;
            break;
        }

        let next_direction_state = state
            .3
            .get(&MoveDirectionCombo(
                direction_came_from,
                current_points_tile.clone(),
            ))
            .unwrap();

        let next_point = next_direction_state.calculate_next_point(&current_point_temp);

        direction_came_from = next_direction_state.prev_direction;
        current_point_temp = next_point;
    }

    let loop_direction = overall_orientation(&loop_chain);
    println!("Loop direction: {:?}", loop_direction);

    /*

                    FF7FSF7F7F7F7F7F---7
                    L|LJ||||||||||||F--J
                    FL-7LJLJ||||||LJL-77
                    F--JF--7||LJLJ7F7FJ-
                    L---JF-JLJ.||-FJLJJ7
                    |F|F-JF---7F7-L7L|7|
                    |FFJF7L7F-JF7|JL---7
                    7-L-JL7||F7|L7F-7F7|
                    L.L7LFJ|||||FJL7||LJ
                    L7JLJL-JLJLJL--JLJ.L

                    FF7FSF7F7F7F7F7F---7
                    L|LJ||||||||||||F--J
                    FL-7LJLJ||||||LJL-77
                    F--JF--7||LJLJIF7FJ-
                    L---JF-JLJIIIIFJLJJ7
                    |F|F-JF---7IIIL7L|7|
                    |FFJF7L7F-JF7IIL---7
                    7-L-JL7||F7|L7F-7F7|
                    L.L7LFJ|||||FJL7||LJ
                    L7JLJL-JLJLJL--JLJ.L
       */

    // North -> South && Vert Pipe = check right
    // West -> East && Hoirz Pipe = check up
    // South -> North && Vert Pipe = check left
    // East - West && Hoirz Pipe = check below
    // record checked point and put it into hashset, check if point has already been visited

    let mut visited_inner_points = HashSet::<PotentialGridLocation>::new();

    let mut all_points_in_loop = HashSet::<PotentialGridLocation>::with_capacity(loop_chain.len() - 1);
    loop_chain.iter().for_each(|item| {
        all_points_in_loop.insert(item.location);
    });

    for point in loop_chain {
        // println!("point: {:?}, tile: {:?}", point.location, point.tile);

        // if point.location.0 == 3 && point.location.1 == 13 {
        //     println!("Ypo");
        // }

        let point_to_check: (isize, isize) = match (&loop_direction, point.tile, point.came_from) {
            (LoopDirection::Counterclockwise, Tile::HorizontalPipe, Direction::East) => (point.location.0 + 1, point.location.1), // check down
            (LoopDirection::Counterclockwise, Tile::HorizontalPipe, Direction::West) => (point.location.0 - 1, point.location.1), // check up
            (LoopDirection::Counterclockwise, Tile::VerticalPipe, Direction::North) => (point.location.0, point.location.1 + 1), // check right
            (LoopDirection::Counterclockwise, Tile::VerticalPipe, Direction::South) => (point.location.0, point.location.1 - 1), // check left

            (LoopDirection::Clockwise, Tile::HorizontalPipe, Direction::East) => (point.location.0 - 1, point.location.1), // check up
            (LoopDirection::Clockwise, Tile::HorizontalPipe, Direction::West) => (point.location.0 + 1, point.location.1), // check down
            (LoopDirection::Clockwise, Tile::VerticalPipe, Direction::North) => (point.location.0, point.location.1 - 1), // check left
            (LoopDirection::Clockwise, Tile::VerticalPipe, Direction::South) => (point.location.0, point.location.1 + 1), // check right

            (LoopDirection::Clockwise, Tile::NorthWestRightAngleBend, Direction::West) => (point.location.0, point.location.1 + 1), // check right



            _ => continue,
        };

        if let Some(check_item) = get_grid_item(&point_to_check, &state) {
            // println!("Came from: {:?}, Point: {:?}, point_item: {:?}, point to check: {:?}, check_item: {:?}", point.came_from, point.location, point.tile, point_to_check, check_item);

            if visited_inner_points.contains(&point_to_check) {
                continue;
            }

            if all_points_in_loop.contains(&point_to_check) {
                continue;
            }

            // what we need to check is whether this check point is not within our own loop chain
            // it will never be outside of the loop, so it can only be a point in our chain or a point that's enclosed
            visited_inner_points.insert(point_to_check);
        }
    }

    println!("visited len: {}", visited_inner_points.len());

    for visited in visited_inner_points {
        println!("{:?}", visited);
    }
}
