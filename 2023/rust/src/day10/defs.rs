use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, Copy)]
pub struct LocationMeta {
    pub came_from: FromDirection,
    pub location: PotentialGridLocation,
    pub tile: Tile,
}


pub type FromDirection = Direction;

#[derive(Eq, Hash, PartialEq)]
pub struct MoveDirectionCombo(pub FromDirection, pub Tile);

pub struct FromToNextDirectionState {
    pub next_direction: Direction,
    pub prev_direction: FromDirection,
}

pub type LineIndex = isize;
pub type IndexOnLine = isize;
pub type PotentialGridLocation = (LineIndex, IndexOnLine);
pub type StartingPoint = PotentialGridLocation;
pub type MoveMappings = HashMap<MoveDirectionCombo, HashSet<Tile>>;
pub type MoveDirectionMappings = HashMap::<MoveDirectionCombo, FromToNextDirectionState>;
pub type GridState = (StartingPoint, HashMap<LineIndex, Vec<Tile>>, MoveMappings, MoveDirectionMappings);

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
