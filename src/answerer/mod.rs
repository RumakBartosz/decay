use std::io::{stdin, Write};

pub struct Answerer {
    message: String
}


impl Answerer {

    pub fn new() -> Answerer {
        Answerer {
            message: String::from("")
        }
    }

    fn retrieve_message(&self) -> String {
        let mut s = String::new();
        stdin().read_line(&mut s)
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

    fn answer_color(&self) -> &str {
        "unimplemented"
    }

    fn answer_move(&self) -> &str {
        "unimplemented"
    }

    fn answer_exit(&self) -> &str {
        "exit"
    }

    pub fn answer(&self) {
        std::io::stdout().flush().unwrap();
        let received_message = self.retrieve_message();

        match received_message.trim() {
            "tbi"    => println!("{}", self.answer_protocol()),
            "tbi v1" => println!("{}", self.answer_version()),
            _        => println!("{}", self.answer_move())
        }
    }
}
