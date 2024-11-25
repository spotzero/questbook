use crate::adventure::*;

pub struct TextRunner {
    adventure: Adventure,
}

impl TextRunner {
    pub fn new(adventure: Adventure) -> TextRunner {
        TextRunner{
            adventure: adventure,
        }
    }

    pub fn start(&mut self) {
        self.adventure.start();
        println!("{}", self.adventure.questbook.title());
        println!("Chapter: {:#?}", self.adventure.chapter);
        loop {
            if self.adventure.state == AdventureState::Ended {
                println!("The End");
                break;
            }
            self.adventure.get_scene();
        }
    }
}