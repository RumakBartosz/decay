
#[derive(Clone, PartialEq, Eq)]
pub enum Mark {
    r,
    R,
    b,
    B,
    o,
    x
}

#[derive(PartialEq)]
pub enum Color {
    Red,
    Blue
}

type Map = Vec<Vec<Mark>>;


pub fn get_red_head(map: &Map) -> (usize, usize) {
    for (y_index, row) in map.iter().enumerate() {
        for (x_index, element) in row.iter().enumerate() {
            if element == &Mark::R {
                return (x_index, y_index)
            }
        }
    }

    (0, 0)
}

pub fn get_blue_head(map: &Map) -> (usize, usize) {
    for (y_index, row) in map.iter().enumerate() {
        for (x_index, element) in row.iter().enumerate() {
            if element == &Mark::B {
                return (x_index, y_index)
            }
        }
    }

    (0, 0)
}

pub fn is_move_up_possible(map: &Map, color: Color) -> bool {
    if color == Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red - 1][x_red] == Mark::x {
            return true
        }
    }

    if color == Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue - 1][x_blue] == Mark::x {
            return true
        }
    }

    false
}

pub fn is_move_down_possible(map: &Map, color: Color) -> bool {
    if color == Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red + 1][x_red] == Mark::x {
            return true
        }
    }

    if color == Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue + 1][x_blue] == Mark::x {
            return true
        }
    }

    false
}


pub fn is_move_left_possible(map: &Map, color: Color) -> bool {
    if color == Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red][x_red - 1] == Mark::x {
            return true
        }
    }

    if color == Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue][x_blue - 1] == Mark::x {
            return true
        }
    }

    false
}


pub fn is_move_right_possible(map: &Map, color: Color) -> bool {
    if color == Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red][x_red + 1] == Mark::x {
            return true
        }
    }

    if color == Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue][x_blue + 1] == Mark::x {
            return true
        }
    }

    false
}

