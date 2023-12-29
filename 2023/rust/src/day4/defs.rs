use std::collections::HashSet;

#[derive(Debug)]
pub struct CardState {
    pub winning_numbers: HashSet<usize>,
    pub my_numbers: HashSet<usize>,
}