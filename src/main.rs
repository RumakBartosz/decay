mod answerer;

fn main() {
    let answerer = answerer::Answerer::new();
    answerer.retrieve_message();
}

