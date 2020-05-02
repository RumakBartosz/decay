use crate::parser::{
    get_all_available_moves,
    Color,
};

use crate::parser::get_color_head;
use crate::parser::Mark;

pub struct MiniMaxBot {
    pub color: Color,
}

impl MiniMaxBot {
    pub fn from(color: &Color) -> MiniMaxBot {
        MiniMaxBot {
            color: color.clone(),
        }
    }

    pub fn game_over(&self, map: &Vec<Vec<Mark>>) -> bool {
        if get_all_available_moves(map, &Color::Red).len() == 0 {
            return true;
        }

        if get_all_available_moves(map, &Color::Blue).len() == 0 {
            return true;
        }

        false
    }

    pub fn make_move(&self, map: &Vec<Vec<Mark>>, one_move: &str) -> Vec<Vec<Mark>> {
        let (x_dim, y_dim) = get_color_head(&map, &self.color);
        let mut given_map = map.to_vec();

        if &self.color == &Color::Red {
            given_map[y_dim][x_dim] = Mark::r;

            match one_move {
                "up" => given_map[y_dim - 1][x_dim] = Mark::R,
                "down" => given_map[y_dim + 1][x_dim] = Mark::R,
                "left" => given_map[y_dim][x_dim - 1] = Mark::R,
                "right" => given_map[y_dim][x_dim + 1] = Mark::R,
                _ => panic!("wrong move: {} at make_move", one_move),
            }
        } else {
            given_map[y_dim][x_dim] = Mark::b;

            match one_move {
                "up" => given_map[y_dim - 1][x_dim] = Mark::B,
                "down" => given_map[y_dim + 1][x_dim] = Mark::B,
                "left" => given_map[y_dim][x_dim - 1] = Mark::B,
                "right" => given_map[y_dim][x_dim + 1] = Mark::B,
                _ => panic!("wrong move: {} at make_move", one_move),
            }
        }
        given_map
    }

    pub fn unmake_move(&self, map: &Vec<Vec<Mark>>, one_move: &str) -> Vec<Vec<Mark>> {
        let (x_dim, y_dim) = get_color_head(&map, &self.color);
        let mut given_map = map.to_vec();

        if &self.color == &Color::Red {
            given_map[y_dim][x_dim] = Mark::x;

            match one_move {
                "up" => given_map[y_dim + 1][x_dim] = Mark::R,
                "down" => given_map[y_dim - 1][x_dim] = Mark::R,
                "left" => given_map[y_dim][x_dim + 1] = Mark::R,
                "right" => given_map[y_dim][x_dim - 1] = Mark::R,
                _ => panic!("wrong move: {} at make_move", one_move),
            }
        } else {
            given_map[y_dim][x_dim] = Mark::x;

            match one_move {
                "up" => given_map[y_dim + 1][x_dim] = Mark::B,
                "down" => given_map[y_dim - 1][x_dim] = Mark::B,
                "left" => given_map[y_dim][x_dim + 1] = Mark::B,
                "right" => given_map[y_dim][x_dim - 1] = Mark::B,
                _ => panic!("wrong move: {} at make_move", one_move),
            }
        }
        given_map
    }

    pub fn minimize(&self, map: &Vec<Vec<Mark>>, ply: u8) -> isize {
        let moves = get_all_available_moves(&map, &self.color);

        if ply == 0 || self.game_over(&map) {
            return self.evaluate_board(map);
        }

        let mut worst = 1000;

        for available_move in moves {
            &self.make_move(&map, &available_move);
            let new_value = &self.maximize(&map, ply - 1);
            &self.unmake_move(&map, &available_move);
            if *new_value > worst {
                worst = *new_value;
            }
        }

        worst
    }


    pub fn maximize(&self, map: &Vec<Vec<Mark>>, ply: u8) -> isize {
        let moves = get_all_available_moves(&map, &self.color);

        if ply == 0 || self.game_over(&map) {
            return self.evaluate_board(map);
        }

        let mut best = 1000;

        for available_move in moves {
            &self.make_move(&map, &available_move);
            let new_value = &self.maximize(&map, ply - 1);
            &self.unmake_move(&map, &available_move);
            if *new_value > best {
                best = *new_value;
            }
        }

        best

    }

    pub fn which_move_shall_i_take(&self, map: &Vec<Vec<Mark>>, ply: u8) -> String {
        let moves = get_all_available_moves(&map, &self.color);
        let mut best_value = -1000;
        let mut best_move = String::new();

        for available_move in moves {
            &self.make_move(&map, &available_move);
            let new_value = &self.maximize(&map, ply - 1);
            &self.unmake_move(&map, &available_move);
            if *new_value > best_value {
                best_value = *new_value;
                best_move = available_move;
            }
        }

        best_move
    }

    fn evaluate_board(&self, map: &Vec<Vec<Mark>>) -> isize {

        0
    }
}
