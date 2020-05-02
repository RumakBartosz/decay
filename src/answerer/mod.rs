use crate::parser::{parse_map, Color, Mark};
use crate::negamax::MiniMaxBot;
use std::io::{stdin, Write};

pub struct Answerer {
    bot: MiniMaxBot,
}

//TODO: change RandBot to MiniMaxBot
impl Answerer {
    pub fn new() -> Answerer {
        Answerer {
            bot: MiniMaxBot::from(&Color::Red),
        }
    }

    fn retrieve_message(&self) -> String {
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("something wrong on message input");

        let message_input: String = s.parse().unwrap();

        message_input
    }

    fn answer_protocol(&self) -> &str {
        "tbi ok"
    }

    fn answer_version(&self) -> &str {
        "tbi v1 ok"
    }

    fn set_color(&mut self, color: Color) {
        self.bot.color = color
    }

    fn answer_color(&self) -> &str {
        "color ok"
    }

    fn answer_move(&self, map: &Vec<Vec<Mark>>) -> String {
        self.bot.which_move_shall_i_take(map, 9)
    }

    pub fn answer(&mut self) {
        std::io::stdout().flush().unwrap();
        let received_message = self.retrieve_message();

        match received_message.trim() {
            "tbi" => println!("{}", self.answer_protocol()),
            "tbi v1" => println!("{}", self.answer_version()),
            "color red" => {
                self.set_color(Color::Red);
                println!("{}", self.answer_color());
            }
            "color blue" => {
                self.set_color(Color::Blue);
                println!("{}", self.answer_color());
            }
            _ => {
                let move_str: &str = received_message.trim().split(' ').collect::<Vec<&str>>()[1];
                let map: Vec<Vec<Mark>> = parse_map(move_str);
                println!("{}", self.answer_move(&map))
            }
        }
    }
}
