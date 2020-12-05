#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    /// Create an empty tile
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    /// Create a filled, wall tile
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    /// Creates a new instance of Rect
    ///
    /// `x: i32` = The x position of the rect
    ///
    /// `y: i32` = The y position of the rect
    ///
    /// `w: i32` = The width of the rect
    ///
    /// `h: i32` = The height of the rect
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    /// Returns the center of the Rect
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

/// A bunch if utility functions for making and generating the map
pub mod map_util {
    use rand::Rng;
    use std::cmp::*;
    pub type Map = Vec<Vec<super::Tile>>;

    /// Takes a rect and places it in the map
    fn create_room(room: super::Rect, map: &mut Map) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                map[x as usize][y as usize] = super::Tile::empty();
            }
        }
    }

    /// Creates a horizontal tunnel
    fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
        for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
            map[x as usize][y as usize] = super::Tile::empty();
        }
    }

    /// Creates a vertical tunnel
    fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
        for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
            map[x as usize][y as usize] = super::Tile::empty();
        }
    }
    /// Fills the map
    pub fn make_map(
        map_width: i32,
        map_height: i32,
        room_min_size: i32,
        room_max_size: i32,
        max_rooms: i32,
        player: &mut crate::object::Object,
    ) -> Map {
        let mut map = vec![vec![super::Tile::wall(); map_height as usize]; map_width as usize];
        let mut rooms = vec![];
        for _ in 0..max_rooms {
            let w = rand::thread_rng().gen_range(room_min_size, room_max_size + 1);
            let h = rand::thread_rng().gen_range(room_min_size, room_max_size + 1);
            let x = rand::thread_rng().gen_range(0, map_width - w);
            let y = rand::thread_rng().gen_range(0, map_height - h);
            let new_room = super::Rect::new(x, y, w, h);
            let failed = rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));
            if !failed {
                create_room(new_room, &mut map);

                let (new_x, new_y) = new_room.center();

                if rooms.is_empty() {
                    player.x = new_x;
                    player.y = new_y;
                } else {
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    if rand::random() {
                        create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                        create_v_tunnel(prev_y, new_y, new_x, &mut map);
                    } else {
                        create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                        create_h_tunnel(prev_x, new_x, new_y, &mut map);
                    }
                }

                rooms.push(new_room)
            }
        }

        map
    }
}
