use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use tcod::map::{FovAlgorithm, Map as FovMap};

mod game;
mod map;
mod object;
use object::Object;

// Actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// Size of the game map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

// FOV parameters
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

// Parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIX_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;
const PLAYER: usize = 0;

// Colours
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 50,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

// Cap the framerate at 24 FPS
const LIMIT_FPS: i32 = 24;

// Renders everythng
fn render_all(
    tcod: &mut game::Tcod,
    game: &mut game::Game,
    objects: &[Object],
    fov_recompute: bool,
) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if fov_recompute {
                let player = &objects[PLAYER];
                tcod.fov
                    .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
            }

            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_LIGHT_WALL,
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };

            let explored = &mut game.map[x as usize][y as usize].explored;

            if visible {
                *explored = true;
            }
            if *explored {
                tcod.con
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }

    // Blit the contents of "con" and the root console to present it
    blit(
        &tcod.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn handle_keys(
    tcod: &mut game::Tcod,
    objects: &mut Vec<Object>,
    game: &game::Game,
) -> object::PlayerAction {
    let key = tcod.root.wait_for_keypress(true);
    match (key, key.text(), objects[PLAYER].alive) {
        (Key { code: Up, .. }, _, true) => {
            object::player_util::player_move_or_attack(PLAYER, 0, -1, &game, objects);
            object::PlayerAction::TookTurn
        }
        (Key { code: Down, .. }, _, true) => {
            object::player_util::player_move_or_attack(PLAYER, 0, 1, &game, objects);
            object::PlayerAction::TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            object::player_util::player_move_or_attack(PLAYER, -1, 0, &game, objects);
            object::PlayerAction::TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            object::player_util::player_move_or_attack(PLAYER, 1, 0, &game, objects);
            object::PlayerAction::TookTurn
        }
        _ => object::PlayerAction::DidntTakeTurn,
    }
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike")
        .init();
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let fov_map: FovMap = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    let mut tcod = game::Tcod {
        root: root,
        con: con,
        fov: fov_map,
    };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Object::new(0, 0, '@', BLACK, "player", true);

    player.fighter = Some(object::Fighter {
        max_hp: 30,
        hp: 30,
        defense: 2,
        power: 5,
    });

    player.alive = true;

    let mut objects = vec![player];
    let mut game_obj = game::Game {
        // Generate map
        map: map::map_util::make_map(
            MAP_WIDTH,
            MAP_HEIGHT,
            ROOM_MIX_SIZE,
            ROOM_MAX_SIZE,
            MAX_ROOMS,
            MAX_ROOM_MONSTERS,
            &mut objects,
            PLAYER,
        ),
    };

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game_obj.map[x as usize][y as usize].block_sight,
                !game_obj.map[x as usize][y as usize].blocked,
            );
        }
    }

    let mut previous_player_position = (-1, -1);
    while !tcod.root.window_closed() {
        tcod.con.set_default_background(BLACK);
        tcod.con.clear();

        let fov_recompute = previous_player_position != objects[PLAYER].pos();
        render_all(&mut tcod, &mut game_obj, &mut objects, fov_recompute);

        tcod.root.flush();

        previous_player_position = objects[PLAYER].pos();
        let player_action = handle_keys(&mut tcod, &mut objects, &game_obj);

        // Allow monsters to take their turns
        if objects[PLAYER].alive && player_action != object::PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    game_obj.ai_take_turn(id, PLAYER, &tcod, &mut objects);
                }
            }
        }
    }
}
