use crate::book::*;
use std::collections::HashSet;

pub struct Adventure {
    pub questbook: Questbook,
    pub tags: HashSet<String>,
    pub inventory: HashSet<String>,
    pub statuses: HashSet<String>,
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
            tags: HashSet::new(),
            inventory: HashSet::new(),
            statuses: HashSet::new(),
            chapter: None,
            scene: None,
            state: AdventureState::Init,
        }
    }

    pub fn start(&mut self) {
        self.chapter = self.get_chapter();
        match self.chapter {
            Some(_) => {
                self.state = AdventureState::Started;
                self.scene = self.get_scene();
            },
            None => self.state = AdventureState::Ended,
        }
    }

    pub fn get_chapter(&self) -> Option<String> {
        let chapters = self.get_chapters();
        if chapters.is_none() {
            None
        } else {
            chapters.unwrap().pop()
        }
    }

    pub fn get_chapters(&self) -> Option<Vec<String>> {
        if self.questbook.chapters.is_empty() {
            return None;
        }
        let mut chapters: Vec<String> = Vec::new();
        for (id, chapter) in self.questbook.chapters.iter() {
            match &chapter.requirements {
                Some(req) => {
                    if self.check_requirements(&req) {
                        chapters.push(id.clone());
                    }
                },
                None => chapters.push(id.clone()),
            }
        }
        if chapters.is_empty() {
            None
        } else {
            Some(chapters)
        }
    }

    pub fn get_scene(&mut self) -> Option<String> {
        match self.get_scenes() {
            Some(mut scenes) => scenes.pop(),
            None => None,
        }
    }

    pub fn get_scenes(&self) -> Option<Vec<String>> {
        None
    }

    /**
     * Get the available decisions a player can make.
     */
    pub fn get_decisions(&self) -> Option<Vec<String>> {
        if self.chapter.is_none() || self.scene.is_none() {
            return None;
        }

        None
    }

    /**
     * Given requirements, see if they're met.
     */
    fn check_requirements(&self, req: &Vec<Requirement>) -> bool {
        req.iter().all(|r|
            match r {
                Requirement::Require(val) => self.inventory.contains(val) || self.statuses.contains(val) || self.tags.contains(val),
                Requirement::Refuse(val) => !(self.inventory.contains(val) || self.statuses.contains(val) || self.tags.contains(val)),
            }
        )
    }
}