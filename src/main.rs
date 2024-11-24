use std::env;
use questbook::book;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let questbook = book::load_questbook(file);
    println!("{:#?}", questbook);
}
