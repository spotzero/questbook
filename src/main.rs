use std::env;
use questbook::book::load_questbook;
use questbook::adventure::Adventure;
use questbook::textrunner::TextRunner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let questbook = load_questbook(file);
    let adventure = Adventure::new(questbook);
    let mut textrunner = TextRunner::new(adventure);
    textrunner.start();
}
