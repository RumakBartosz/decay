use std::io::{stdin, Write};
#[path = "../rand_bot/mod.rs"]
mod bot;
#[path = "../parser/mod.rs"]
mod parser;
use bot::parser::{parse_map, Color, Mark};
use bot::RandBot;

pub struct Answerer {
    bot: RandBot,
}

impl Answerer {
    pub fn new() -> Answerer {
        Answerer {
            bot: RandBot::from(Vec::new(), Color::Red),
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

    fn answer_move(&self) -> &str {
        self.bot.return_available_move()
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
                self.bot.map = map;
                println!("{}", self.answer_move())
            }
        }
    }
}
