use crate::map::map_util;
use crate::object;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};

pub struct Game {
    pub map: map_util::Map,
}

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub fov: FovMap,
}

impl Game {
    pub fn ai_take_turn(
        &self,
        monster_id: usize,
        player_id: usize,
        tcod: &Tcod,
        objects: &mut [object::Object],
    ) {
        let (monster_x, monster_y) = objects[monster_id].pos();
        if tcod.fov.is_in_fov(monster_x, monster_y) {
            if objects[monster_id].distance_to(&objects[player_id]) >= 2.0 {
                let (player_x, player_y) = objects[player_id].pos();
                object::player_util::move_towards(
                    monster_id, player_x, player_y, &self.map, objects,
                );
            } else if objects[player_id].fighter.map_or(false, |f| f.hp > 0) {
                let monster = &objects[monster_id];
                println!("A {} tried to attack you!", monster.name)
            }
        }
    }
}
