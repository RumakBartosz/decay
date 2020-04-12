#[path = "../parser/mod.rs"] pub mod parser;
use parser::{Mark, Color,
             is_move_up_possible, is_move_down_possible,
             is_move_left_possible, is_move_right_possible};

use rand;
use rand::seq::SliceRandom;


pub struct RandBot {
    pub color: Color,
    pub map: Vec<Vec<Mark>>
}


impl RandBot {
    pub fn from(color: Color, map: Vec<Vec<Mark>>) -> RandBot {
        RandBot {
            color,
            map
        }
    }

    fn get_all_available_moves(&self) -> Vec<&str> {
        let mut available_move: Vec<&str> = Vec::new();
        if is_move_up_possible(&self.map, &self.color) {
            available_move.push("up");
        }
        if is_move_down_possible(&self.map, &self.color) {
            available_move.push("down");
        }
        if is_move_left_possible(&self.map, &self.color) {
            available_move.push("left");
        }
        if is_move_right_possible(&self.map, &self.color) {
            available_move.push("right");
        }

        available_move
    }

    pub fn return_available_move(&self) -> &str {
        let possible_moves = self.get_all_available_moves();
        let mut rng = rand::thread_rng();

        possible_moves.choose(&mut rng).unwrap()
    }
}
