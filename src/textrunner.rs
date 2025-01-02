use crate::adventure::*;
use std::io;

pub struct TextRunner {
    dev_mode: bool,
    adventure: Adventure,
}

impl TextRunner {
    pub fn new(adventure: Adventure, dev_mode: bool) -> TextRunner {
        TextRunner{
            dev_mode: dev_mode,
            adventure: adventure,
        }
    }

    pub fn start(&mut self) {
        self.adventure.start();

        println!("{}\n", self.adventure.questbook.story.title);
        let chapter = self.adventure.chapter.as_ref().unwrap();
        println!("{}", self.adventure.questbook.chapters.get(chapter).unwrap().title);

        loop {
            if self.adventure.state == AdventureState::Ended {
                println!("The End");
                break;
            }

            let cur_scenes = self.adventure.get_scenes();
            let cur_decisions = self.adventure.get_decisions();

            if self.dev_mode {
                println!("DEV - Current chapter: {:#?}", self.adventure.chapter);
                println!("DEV - Current scene: {:#?}", self.adventure.scene);
                println!("DEV - Current inventory: {:#?}", self.adventure.inventory);
                println!("DEV - Current statuses: {:#?}", self.adventure.statuses);
                println!("DEV - Available scenes: {:#?}", cur_scenes);
                println!("DEV - Available decisions: {:#?}", cur_decisions);
            }

            self.display_scene();

            println!("Enter your action: ");
            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            command = command.trim().to_ascii_lowercase();
            if command == "exit" || command == "quit" {
                println!("Exiting game");
                break;
            }

            if cur_scenes.contains(&command) {
                self.adventure.change_scene(&command, true);
            } else if cur_decisions.contains(&command) {
                let output = self.adventure.make_decision(&command);
                println!("{:#?}", output);
            } else {
                println!("Invalid action: {}", command);
            }

        }
        if self.dev_mode {
            println!("{:#?}", self.adventure.log);
        }
        println!("Game over");
    }

    fn change_chapter(&mut self) {

    }

    fn display_scene(&mut self) {

    }

    fn make_decision(&mut self, decision: &str) {

    }
}