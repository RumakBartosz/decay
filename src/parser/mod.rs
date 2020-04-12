
#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn get_color_head(map: &Map, color: Color) -> (usize, usize) {
    if color == Color::Red {
        get_red_head(map)
    }
    else {
        get_blue_head(map)
    }
}

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

pub fn is_move_up_possible(map: &Map, color: &Color) -> bool {
    if color == &Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red - 1][x_red] == Mark::x {
            return true
        }
    }

    if color == &Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue - 1][x_blue] == Mark::x {
            return true
        }
    }

    false
}

pub fn is_move_down_possible(map: &Map, color: &Color) -> bool {
    if color == &Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red + 1][x_red] == Mark::x {
            return true
        }
    }

    if color == &Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue + 1][x_blue] == Mark::x {
            return true
        }
    }

    false
}


pub fn is_move_left_possible(map: &Map, color: &Color) -> bool {
    if color == &Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red][x_red - 1] == Mark::x {
            return true
        }
    }

    if color == &Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue][x_blue - 1] == Mark::x {
            return true
        }
    }

    false
}


pub fn is_move_right_possible(map: &Map, color: &Color) -> bool {
    if color == &Color::Red {
        let (x_red, y_red) = get_red_head(&map);

        if map[y_red][x_red + 1] == Mark::x {
            return true
        }
    }

    if color == &Color::Blue {
        let (x_blue, y_blue) = get_blue_head(&map);

        if map[y_blue][x_blue + 1] == Mark::x {
            return true
        }
    }

    false
}

pub fn break_on_slash(move_string: &str) -> Vec<Vec<char>> {
    let mut row: Vec<char> = Vec::new();
    let mut whole_map: Vec<Vec<char>> = Vec::new();

    for element in move_string.chars() {
        if element == '/' {
            whole_map.push(row);
            row = Vec::new();
        }
        else {
            row.push(element);
        }
    }
    whole_map.push(row);
    whole_map
}

fn number_of_spaces(num: u32) -> Vec<char> {
    let mut spaces_number: Vec<char> = Vec::new();
    for _ in 0..num {
        spaces_number.push(' ');
    }
    spaces_number
}

pub fn number_to_spaces(map_row: Vec<char>) -> Vec<char> {
    let mut returned_row: Vec<char> = Vec::new();
    let mut tens: u32 = 0;
    let mut ones: u32 = 0;
    let mut spaces: Vec<char> = Vec::new();

    for element in &map_row {
        if element.is_digit(10) && tens == 0 {
            tens = element.to_digit(10).unwrap();
        }
        else if element.is_digit(10) && tens != 0 {
            ones = element.to_digit(10).unwrap();
            spaces = number_of_spaces(tens * 10 + ones);
            tens = 0;
            returned_row.append(&mut spaces);
        }
        else if tens != 0 {
            spaces = number_of_spaces(tens);
            tens = 0;
            returned_row.append(&mut spaces);
            returned_row.push(*element);
        }
        else {
            returned_row.push(*element);
        }

    }

    returned_row
}

fn markify_row(string_map_row: Vec<char>) -> Vec<Mark> {
    let mut row: Vec<Mark> = Vec::new();

    for element in string_map_row {
        if element == 'o' {
            row.push(Mark::o);
        }
        else if element == ' ' {
            row.push(Mark::x);
        }
        else if element == 'r' {
            row.push(Mark::r);
        }
        else if element == 'R' {
            row.push(Mark::R);
        }
        else if element == 'b' {
            row.push(Mark::b);
        }
        else if element == 'B' {
            row.push(Mark::B);
        }
    }

    row
}

fn markify_map(string_map: Vec<Vec<char>>) -> Vec<Vec<Mark>> {
    let mut markified_map: Vec<Vec<Mark>> = Vec::new();

    for vector in string_map {
        markified_map.push(markify_row(vector));
    }

    markified_map
}

pub fn parse_map(move_string: &str) -> Vec<Vec<Mark>> {
    let broken_on_slash: Vec<Vec<char>> = break_on_slash(move_string);
    let mut parsed_map: Vec<Vec<Mark>> = Vec::new();

    let mut numbers_changed: Vec<Vec<char>> = Vec::new();

    for row in broken_on_slash {
        numbers_changed.push(number_to_spaces(row));
    }

    parsed_map = markify_map(numbers_changed);

    parsed_map
}
