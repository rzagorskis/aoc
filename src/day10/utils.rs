use std::collections::{HashMap, HashSet};

use crate::io_utils::read_lines_fully;

use super::defs::{FromToNextDirectionState, PotentialGridLocation, Direction, MoveDirectionCombo, FromDirection, Tile, LocationMeta, MoveMappings, StartingPoint, GridState, LineIndex};

pub fn build_directional_move_map() -> HashMap::<MoveDirectionCombo, FromToNextDirectionState> {
    let mut move_map = HashMap::<MoveDirectionCombo, FromToNextDirectionState>::new();

    move_map.insert(
        MoveDirectionCombo(FromDirection::North, Tile::VerticalPipe),
        FromToNextDirectionState {
            next_direction: Direction::South,
            prev_direction: Direction::North,
        },
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::South, Tile::VerticalPipe),
        FromToNextDirectionState {
            next_direction: Direction::North,
            prev_direction: Direction::South,
        },
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::West, Tile::HorizontalPipe),
        FromToNextDirectionState {
            next_direction: Direction::East,
            prev_direction: Direction::West,
        },
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::East, Tile::HorizontalPipe),
        FromToNextDirectionState {
            next_direction: Direction::West,
            prev_direction: Direction::East,
        },
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::North, Tile::NorthEastRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::East,
            prev_direction: Direction::West,
        },
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::East, Tile::NorthEastRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::North,
            prev_direction: Direction::South,
        },
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::North, Tile::NorthWestRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::West,
            prev_direction: Direction::East
        },
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::West, Tile::NorthWestRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::North,
            prev_direction: Direction::South,
        },
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::South, Tile::SouthWestRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::West,
            prev_direction: Direction::East,
        },
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::West, Tile::SouthWestRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::South,
            prev_direction: Direction::North,
        },
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::South, Tile::SouthEastRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::East,
            prev_direction: Direction::West,
        },
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::East, Tile::SouthEastRightAngleBend),
        FromToNextDirectionState {
            next_direction: Direction::South,
            prev_direction: Direction::North,
        },
    );

    return move_map;
}

pub fn build_valid_move_map() -> MoveMappings {
    let mut move_map = MoveMappings::new();

    let mut from_north_directions = HashSet::<Tile>::new();
    from_north_directions.insert(Tile::VerticalPipe);
    from_north_directions.insert(Tile::NorthWestRightAngleBend);
    from_north_directions.insert(Tile::NorthEastRightAngleBend);
    move_map.insert(
        MoveDirectionCombo(FromDirection::North, Tile::VerticalPipe),
        from_north_directions.clone(),
    );

    let mut from_sotuh_directions = HashSet::<Tile>::new();
    from_sotuh_directions.insert(Tile::VerticalPipe);
    from_sotuh_directions.insert(Tile::SouthWestRightAngleBend);
    from_sotuh_directions.insert(Tile::SouthEastRightAngleBend);
    move_map.insert(
        MoveDirectionCombo(FromDirection::South, Tile::VerticalPipe),
        from_sotuh_directions.clone(),
    );

    let mut from_west_directions = HashSet::<Tile>::new();
    from_west_directions.insert(Tile::HorizontalPipe);
    from_west_directions.insert(Tile::NorthWestRightAngleBend);
    from_west_directions.insert(Tile::SouthWestRightAngleBend);
    move_map.insert(
        MoveDirectionCombo(FromDirection::West, Tile::HorizontalPipe),
        from_west_directions.clone(),
    );

    let mut from_east_directions = HashSet::<Tile>::new();
    from_east_directions.insert(Tile::HorizontalPipe);
    from_east_directions.insert(Tile::NorthEastRightAngleBend);
    from_east_directions.insert(Tile::SouthEastRightAngleBend);
    move_map.insert(
        MoveDirectionCombo(FromDirection::East, Tile::HorizontalPipe),
        from_east_directions.clone(),
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::North, Tile::NorthEastRightAngleBend),
        from_east_directions.clone(),
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::East, Tile::NorthEastRightAngleBend),
        from_north_directions.clone(),
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::North, Tile::NorthWestRightAngleBend),
        from_west_directions.clone(),
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::West, Tile::NorthWestRightAngleBend),
        from_north_directions.clone(),
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::South, Tile::SouthWestRightAngleBend),
        from_west_directions.clone(),
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::West, Tile::SouthWestRightAngleBend),
        from_sotuh_directions.clone(),
    );

    move_map.insert(
        MoveDirectionCombo(FromDirection::South, Tile::SouthEastRightAngleBend),
        from_east_directions.clone(),
    );
    move_map.insert(
        MoveDirectionCombo(FromDirection::East, Tile::SouthEastRightAngleBend),
        from_sotuh_directions.clone(),
    );

    return move_map;
}

pub fn get_grid_item<'a>(
    point: &'a PotentialGridLocation,
    grid: &'a GridState,
) -> Option<&'a Tile> {
    if !is_point_within_grid(point) {
        return None;
    }

    return grid
        .1
        .get(&point.0)
        .and_then(|line| line.iter().nth(point.1 as usize));
}

pub const fn is_point_within_grid(point: &PotentialGridLocation) -> bool {
    return point.0 >= 0 && point.1 >= 0;
}

pub fn build() -> GridState {
    let mut starting_point_location: StartingPoint = (0, 0);
    let valid_move_map = build_valid_move_map();
    let direction_map = build_directional_move_map();
    let mut tile_grid = HashMap::<LineIndex, Vec<Tile>>::new();

    read_lines_fully("src/day10/input.txt")
        .iter()
        .enumerate()
        .for_each(|(line_ix, line)| {
            let mut line_tiles = Vec::<Tile>::with_capacity(line.len());
            for (char_ix, char) in line.chars().enumerate() {
                let tile = match char {
                    '|' => Tile::VerticalPipe,
                    '-' => Tile::HorizontalPipe,
                    'L' => Tile::NorthEastRightAngleBend,
                    'J' => Tile::NorthWestRightAngleBend,
                    '7' => Tile::SouthWestRightAngleBend,
                    'F' => Tile::SouthEastRightAngleBend,
                    '.' => Tile::Ground,
                    'S' => {
                        // need to record starting location
                        starting_point_location = (line_ix as isize, char_ix as isize);

                        Tile::StartingPoint
                    }
                    _ => Tile::Unknown,
                };

                line_tiles.push(tile);
            }

            tile_grid.insert(line_ix as isize, line_tiles);
        });

    return (starting_point_location, tile_grid, valid_move_map, direction_map);
}

pub fn build_loop_chain() -> Vec::<LocationMeta> {
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
  
    // now that we have the effective starting point, already step of 1, we need to figure out from the point we're on now
    // which direction we can go next and check the point at next location
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

    return loop_chain;
}