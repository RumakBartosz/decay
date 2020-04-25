use crate::parser::{Color,
             is_move_up_possible, is_move_down_possible,
             is_move_left_possible, is_move_right_possible};

use crate::parser::Mark::{self,x};


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

    pub fn game_over(&self) -> bool {
        if self.get_all_available_moves(Color::Red).len() == 0 {
            return true;
        }

        if self.get_all_available_moves(Color::Blue).len() == 0 {
            return true;
        }

        false
    }

    fn get_all_available_moves(&self, color: Color) -> Vec<&str> {
        let mut available_move: Vec<&str> = Vec::new();
        if is_move_up_possible(&self.map, &color) {
            available_move.push("up");
        }
        if is_move_down_possible(&self.map, &color) {
            available_move.push("down");
        }
        if is_move_left_possible(&self.map, &color) {
            available_move.push("left");
        }
        if is_move_right_possible(&self.map, &color) {
            available_move.push("right");
        }

        available_move
    }

    fn make_move(one_move: &str) -> Vec<Vec<Mark>> {
        vec![vec![x]]
    }

    fn unmake_move(one_move: &str) -> Vec<Vec<Mark>> 
        vec![vec![x]]
    }
}
