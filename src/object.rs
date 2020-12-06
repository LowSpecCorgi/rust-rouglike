use tcod::colors::*;
use tcod::console::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ai {
    Basic,
}

/// `x: i32` = The x position of the object
///
/// `y: i32` = The y position of the object
///
/// `character: char` = The ASCII text character to represent this Object
///
/// `colour: Color` = The colour to use to display this Object with
///
/// `name: String` = The name of this Object
///
/// `blocks: bool` = Sets whether this Object blocks the player or not
///
/// `alive: bool` = Sets whether this Object is alive
#[derive(Clone, Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub character: char,
    pub colour: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
}

impl Object {
    /// Creates a new Object instance
    ///
    /// `x: i32` = The position on the x axis that the Object should be located
    ///
    /// `y: i32` = The position on the y axis that the Object should be located
    ///
    /// `character: char` = The text character that you want to represent your character
    ///
    /// `colour: Color` = The colour this Object is displayed with
    pub fn new(x: i32, y: i32, character: char, colour: Color, name: &str, blocks: bool) -> Self {
        Object {
            x: x,
            y: y,
            character: character,
            colour: colour,
            name: name.into(),
            blocks: blocks,
            alive: false,
            fighter: None,
            ai: None,
        }
    }

    /// Sets the Object's position
    ///
    /// `x : i32` = The x position that you want to move the Object to
    ///
    /// `y : i32` = The y position that you want to move the Object to
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Gets the Object's position
    ///
    /// Returns `(i32, i32)`
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    /// Sets the colour and draws the character that represents this Object at it's position
    ///
    /// `con: &mut dyn Console` = Pass the console you want to draw the character to, here
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.colour);
        con.put_char(self.x, self.y, self.character, BackgroundFlag::None);
    }

    /// Returns the distance to another object
    ///
    /// `other: &Object` = The other object
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}

pub mod player_util {
    use crate::game::Game;
    use crate::map;
    use crate::object::Object;

    /// Move the Player by a given amount
    ///
    /// `dx: i32` = The amount you want to move the Player on the x axis
    ///
    /// `dy: i32` = The amount you want to move the Player on the y axis
    pub fn move_by(id: usize, dx: i32, dy: i32, map: &map::map_util::Map, objects: &mut [Object]) {
        let (x, y) = objects[id].pos();
        if !map::map_util::is_blocked(x + dx, y + dy, map, objects) {
            objects[id].set_pos(x + dx, y + dy)
        }
    }

    /// Move the Player by a given amount and attack if monster resides in that position
    ///
    /// `player_id: usize` = The ID of the player
    ///
    /// `dx: i32` = The amount you want to move the Player on the x axis
    ///
    /// `dy: i32` = The amount you want to move the Player on the y axis
    ///
    /// `game: &Game` = The actual Game object
    ///
    /// `objects: &mut [Object]` = The vector of objects
    pub fn player_move_or_attack(
        player_id: usize,
        dx: i32,
        dy: i32,
        game: &Game,
        objects: &mut [Object],
    ) {
        let x = objects[player_id].x + dx;
        let y = objects[player_id].y + dy;

        let target_id = objects.iter().position(|object| object.pos() == (x, y));

        match target_id {
            Some(target_id) => {
                println!("You tried to attack {}!", objects[target_id].name)
            }
            None => move_by(player_id, dx, dy, &game.map, objects),
        }
    }

    /// Moves an object towards a specified position
    ///
    /// `id: usize` = The ID of the Object that you want to move
    ///
    /// `target_x: i32` = The x position that you want to move the Object towards
    ///
    /// `target_x: i32` = The y position that you want to move the Object towards
    ///
    /// `map: &map::map_util::Map` = The actual Game object
    ///
    /// `objects: &mut [Object]` = THe vector that stores all objects
    pub fn move_towards(
        id: usize,
        target_x: i32,
        target_y: i32,
        map: &map::map_util::Map,
        objects: &mut [Object],
    ) {
        let dx = target_x - objects[id].x;
        let dy = target_y - objects[id].y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        let dx = (dx as f32 / distance).round() as i32;
        let dy = (dy as f32 / distance).round() as i32;
        move_by(id, dx, dy, map, objects);
    }
}
