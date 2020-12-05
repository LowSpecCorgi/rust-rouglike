use crate::game;
use tcod::colors::*;
use tcod::console::*;

pub struct Object {
    pub x: i32,
    pub y: i32,
    pub character: char,
    pub colour: Color,
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
    pub fn new(x: i32, y: i32, character: char, colour: Color) -> Self {
        Object {
            x,
            y,
            character,
            colour,
        }
    }

    /// Move the Object by a given amount
    ///
    /// `dx: i32` = The amount you want to move the character on the x axis
    ///
    /// `dy: i32` = The amount you want to move the character on the y axis
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &game::Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    /// Set the colour and draws the character that represents this Object at it's position
    ///
    /// `con: &mut dyn Console` = Pass the console you want to draw the character to here
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_background(self.colour);
        con.put_char(self.x, self.y, self.character, BackgroundFlag::None);
    }
}
