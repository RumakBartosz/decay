mod answerer;
use answerer::Answerer;

fn main() {
    let mut answerer = Answerer::new();

    loop {
        answerer.answer();
    }
}


