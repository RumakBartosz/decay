use std::io::stdin;

pub struct Answerer {
    message: String
}


impl Answerer {

    pub fn new() -> Answerer {
        Answerer {
            message: String::from("")
        }
    }

    pub fn retrieve_message(&self) -> String {
        let mut s = String::new();
        stdin().read_line(&mut s)
            .expect("something wrong on message input");

        let message_input: String = s.parse().unwrap();

        println!("{:?}", message_input);

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

}
