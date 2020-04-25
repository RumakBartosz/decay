use crate::parser::{Color,
             is_move_up_possible, is_move_down_possible,
             is_move_left_possible, is_move_right_possible};

use crate::parser::Mark;
use crate::parser::get_color_head;


pub struct MiniMaxBot {
    map: Vec<Vec<Mark>>,
    color: Color
}

impl MiniMaxBot {
    pub fn from(map: &Vec<Vec<Mark>>, color: &Color) -> MiniMaxBot {
        MiniMaxBot {
            map: map.clone(),
            color: color.clone()
        }
    }

    pub fn get_map(&self) -> &Vec<Vec<Mark>> {
        &self.map
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

    pub fn make_move(&mut self, one_move: &str) {
        let (x_dim, y_dim) = get_color_head(&self.map, &self.color);

        if &self.color == &Color::Red {
            self.map[y_dim][x_dim] = Mark::r;

            match one_move {
                "up" => self.map[y_dim - 1][x_dim] = Mark::R,
                "down" => self.map[y_dim + 1][x_dim] = Mark::R,
                "left" => self.map[y_dim][x_dim - 1]= Mark::R,
                "right" => self.map[y_dim][x_dim + 1] = Mark::R,
                _ => panic!("wrong move: {} at make_move", one_move)
            }
        }
        else {
            self.map[y_dim][x_dim] = Mark::b;

            match one_move {
                "up" => self.map[y_dim - 1][x_dim] = Mark::B,
                "down" => self.map[y_dim + 1][x_dim] = Mark::B,
                "left" => self.map[y_dim][x_dim - 1] = Mark::B,
                "right" => self.map[y_dim][x_dim + 1] = Mark::B,
                _ => panic!("wrong move: {} at make_move", one_move)
            }
        }
    }

    pub fn unmake_move(&mut self, one_move: &str) {
        let (x_dim, y_dim) = get_color_head(&self.map, &self.color);

        if &self.color == &Color::Red {
            self.map[y_dim][x_dim] = Mark::x;

            match one_move {
                "up" => self.map[y_dim - 1][x_dim] = Mark::R,
                "down" => self.map[y_dim + 1][x_dim] = Mark::R,
                "left" => self.map[y_dim][x_dim - 1]= Mark::R,
                "right" => self.map[y_dim][x_dim + 1] = Mark::R,
                _ => panic!("wrong move: {} at make_move", one_move)
            }
        }
        else {
            self.map[y_dim][x_dim] = Mark::x;

            match one_move {
                "up" => self.map[y_dim + 1][x_dim] = Mark::B,
                "down" => self.map[y_dim - 1][x_dim] = Mark::B,
                "left" => self.map[y_dim][x_dim + 1] = Mark::B,
                "right" => self.map[y_dim][x_dim - 1] = Mark::B,
                _ => panic!("wrong move: {} at make_move", one_move)
            }
        }
    }
}
