use crate::adventure::*;
use std::io;

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

        loop {
            if self.adventure.state == AdventureState::Ended {
                println!("The End");
                break;
            }

            println!("Current chapter: {:#?}", self.adventure.chapter);
            println!("Current scene: {:#?}", self.adventure.scene);
            println!("Current inventory: {:#?}", self.adventure.inventory);
            println!("Current statuses: {:#?}", self.adventure.statuses);

            let cur_scenes = self.adventure.get_scenes();
            println!("Available scenes: {:#?}", cur_scenes);

            let cur_decisions = self.adventure.get_decisions();
            println!("Available decisions: {:#?}", cur_decisions);
            println!("Enter your action: ");
            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            command = command.trim().to_ascii_lowercase();
            if command == "exit" || command == "quit" {
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
        println!("{:#?}", self.adventure.log);
    }
}