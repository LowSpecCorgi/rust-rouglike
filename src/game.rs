use crate::map;

pub type Map = Vec<Vec<map::Tile>>;

pub struct Game {
    pub map: Map,
}
