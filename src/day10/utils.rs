use std::collections::{HashMap, HashSet};

use crate::io_utils::read_lines_fully;


#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Tile {
    HorizontalPipe,          // - : from west -> left-right, from east -> right-left
    VerticalPipe,            // | : from north -> down, from south -> up
    NorthEastRightAngleBend, // L : from east -> left-up, from north -> down-right
    NorthWestRightAngleBend, // J : from west -> right-up, from north -> down-left
    SouthWestRightAngleBend, // 7 : from west -> right-down, from south -> up-left
    SouthEastRightAngleBend, // F : from east -> left-down, from south -> up-right
    Ground,
    StartingPoint,
    Unknown,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub type FromDirection = Direction;

#[derive(Eq, Hash, PartialEq)]
pub struct MoveDirectionCombo(pub FromDirection, pub Tile);

pub struct FromToNextDirectionState {
    next_direction: Direction,
    pub prev_direction: FromDirection,
}

type LineIndex = isize;
type IndexOnLine = isize;
pub type PotentialGridLocation = (LineIndex, IndexOnLine);
type StartingPoint = PotentialGridLocation;
type MoveMappings = HashMap<MoveDirectionCombo, HashSet<Tile>>;
type MoveDirectionMappings = HashMap::<MoveDirectionCombo, FromToNextDirectionState>;
type GridState = (StartingPoint, HashMap<LineIndex, Vec<Tile>>, MoveMappings, MoveDirectionMappings);

impl FromToNextDirectionState {
    pub fn calculate_next_point(&self, from_location: &PotentialGridLocation) -> PotentialGridLocation {
        if self.next_direction == Direction::North {
            return (from_location.0 - 1, from_location.1);
        } else if self.next_direction == Direction::South {
            return (from_location.0 + 1, from_location.1);
        } else if self.next_direction == Direction::East {
            return (from_location.0, from_location.1 + 1);
        } else {
            return (from_location.0, from_location.1 - 1);
        }
    }
}

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
