use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}