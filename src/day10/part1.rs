/*
--- Day 10: Pipe Maze ---

You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....

If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....

In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF

In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ

If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....

You can count the distance each tile in the loop is from the starting point like this:

.....
.012.
.1.3.
.234.
.....

In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...

Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?

*/

use crate::day10::utils::{build, FromDirection, get_grid_item, MoveDirectionCombo, Tile};

const EXPECTED_ANSWER: i32 = 6846;

pub fn run() {
  let state = build();

  println!("{:?}", state.0);

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

              println!(
                  "Came from: {:?}, next moves: {:?}",
                  next_point.0.clone(),
                  next_moves
              );

              if let Some(next_moves) = next_moves {
                  if next_moves.contains(item) {
                      return true;
                  }
              }
          }

          return false;
      })
      .unwrap();

  let mut steps_taken_for_loop = 1; // farthest point = loop steps / 2

  // now that we have the effective starting point, already step of 1, we need to figure out from the point we're on now
  // which direction we can go next and check the point at next location
  let mut looping = true;
  let mut direction_came_from = current_point.0.clone();
  let mut current_point_temp = current_point.1.clone();
  while looping {
      let current_points_tile = get_grid_item(&current_point_temp, &state).unwrap();

      if *current_points_tile == Tile::StartingPoint {
          looping = false;
          break;
      }

      let next_direction_state = state.3.get(&MoveDirectionCombo(
          direction_came_from,
          current_points_tile.clone(),
      )).unwrap();

      let next_point = next_direction_state.calculate_next_point(&current_point_temp);

      direction_came_from = next_direction_state.prev_direction;
      current_point_temp = next_point;

      steps_taken_for_loop += 1;
  }

  println!("{}", steps_taken_for_loop / 2);
}
