use crate::book::*;

pub struct Adventure {
    pub questbook: Questbook,
    pub consequences: Vec<String>,
    pub inventory: Vec<String>,
    pub statuses: Vec<String>,
    pub chapter: Option<String>,
    pub scene: Option<String>,
    pub state: AdventureState,
}

#[derive(PartialEq)]
pub enum AdventureState {
    Init,
    Started,
    Ended,
}

impl Adventure {
    pub fn new(questbook: Questbook) -> Adventure {
        Adventure {
            questbook: questbook,
            consequences: Vec::new(),
            inventory: Vec::new(),
            statuses: Vec::new(),
            chapter: None,
            scene: None,
            state: AdventureState::Init,
        }
    }

    pub fn start(&mut self) {
        self.chapter = self.next_chapter();
        match self.chapter {
            Some(_) => {
                self.state = AdventureState::Started;
                self.scene = self.next_scene();
            },
            None => self.state = AdventureState::Ended,
        }
    }

    pub fn next_chapter(&self) -> Option<String> {
        None
    }

    pub fn next_scene(&self) -> Option<String> {
        None
    }

    pub fn get_scene(&mut self) {
        
    }
}