#[derive(Debug)]
pub struct GameSet {
    pub red: Option<usize>,
    pub green: Option<usize>,
    pub blue: Option<usize>
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<GameSet>,
    pub ref_line: String,
}
