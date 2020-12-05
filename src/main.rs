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

struct Tcod {
    root: Root,
    con: Offscreen,
    fov: FovMap,
}

// Renders everythng
fn render_all(tcod: &mut Tcod, game: &game::Game, objects: &[Object], fov_recompute: bool) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if fov_recompute {
                let player = &objects[0];
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
            tcod.con
                .set_char_background(x, y, color, BackgroundFlag::Set);
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

fn handle_keys(tcod: &mut Tcod, player: &mut Object, game: &game::Game) {
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        _ => {}
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
    let mut tcod = Tcod {
        root: root,
        con: con,
        fov: fov_map,
    };

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(0, 0, '@', WHITE);

    let mut objects = [player];

    let game_obj = game::Game {
        // Generate map
        map: map::map_util::make_map(
            MAP_WIDTH,
            MAP_HEIGHT,
            ROOM_MIX_SIZE,
            ROOM_MAX_SIZE,
            MAX_ROOMS,
            &mut objects[0],
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

        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut tcod, &game_obj, &mut objects, fov_recompute);

        tcod.root.flush();

        previous_player_position = (player.x, player.y);
        let keys = handle_keys(&mut tcod, &mut objects[0], &game_obj);
    }
}
