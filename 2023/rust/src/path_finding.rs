use std::collections::HashSet;

use aoc_2023::Point;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct StartGoal {
    pub start: Point,
    pub goal: Point
}

pub fn find_all_unique_start_goal_points(points: &Vec<Point>) -> HashSet<StartGoal> {
    let mut set = HashSet::<StartGoal>::new();

    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            set.insert(StartGoal{ start: *point1, goal: *point2 });
        }
    }

    return set;
}