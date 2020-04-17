#[path = "../parser/mod.rs"] pub mod parser;
use parser::{Mark, Color,
             is_move_up_possible, is_move_down_possible,
             is_move_left_possible, is_move_right_possible};

use parser::Mark::*;


pub struct MiniMaxBot {
    map: Vec<Vec<Mark>>,
    color: Color
}

impl MiniMaxBot {
    pub fn from(map: Vec<Vec<Mark>>, color: Color) -> MiniMaxBot {
        MiniMaxBot {
            map,
            color
        }
    }

    fn game_over(&self) -> bool {
        false
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

        if available_move.len() == 0 {
            available_move.push("up");
        }

        available_move
    }

    fn make_move(one_move: &str) -> Vec<Vec<Mark>> {
        vec![vec![x]]
    }

    fn unmake_move(one_move: &str) -> Vec<Vec<Mark>> {
        vec![vec![x]]
    }
}
