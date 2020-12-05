use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;
mod game;
mod map;
mod object;
pub use object::Object;

// Actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// Size of the game map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 50,
};

// Cap the framerate at 24 FPS
const LIMIT_FPS: i32 = 24;

struct Tcod {
    root: Root,
    con: Offscreen,
}

// Fills the map
fn make_map() -> game::Map {
    let mut map = vec![vec![map::Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    map[30][22] = map::Tile::wall();
    map[50][22] = map::Tile::wall();
    map
}

// Renders everythng
fn render_all(tcod: &mut Tcod, game: &game::Game, objects: &[Object]) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    for object in objects {
        object.draw(&mut tcod.con);
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
        .title("Rust roguelike @_@")
        .init();
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod { root, con };
    tcod::system::set_fps(LIMIT_FPS);

    let game_obj = game::Game {
        // Generate map
        map: make_map(),
    };

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    let mut objects = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.set_default_background(BLACK);
        tcod.con.clear();

        render_all(&mut tcod, &game_obj, &mut objects);

        tcod.root.flush();
        let keys = handle_keys(&mut tcod, &mut objects[0], &game_obj);
    }
}
