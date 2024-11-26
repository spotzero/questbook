use crate::book::*;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Adventure {
    pub questbook: Questbook,
    pub tags: HashSet<String>,
    pub inventory: HashSet<String>,
    pub statuses: HashSet<String>,
    pub chapter: Option<String>,
    pub scene: Option<String>,
    pub counters: HashMap<String, i32>,
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
        let mut counters: HashMap<String, i32> = HashMap::new();
        for counter in questbook.counters.iter() {
            counters.insert(counter.0.clone(), counter.1.value);
        }
        Adventure {
            questbook: questbook,
            tags: HashSet::new(),
            inventory: HashSet::new(),
            statuses: HashSet::new(),
            chapter: None,
            scene: None,
            state: AdventureState::Init,
            counters: counters,
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

    /**
     * Returns the latest chapter the player can access.
     */
    pub fn get_chapter(&self) -> Option<String> {
        let chapters = self.get_chapters();
        if chapters.is_empty() {
            return None;
        } else {
            for chapter in self.questbook.story.chapters.iter().rev() {
                if chapters.contains(chapter) {
                    return Some(chapter.clone());
                }
            }
        }
        return None;
    }

    /**
     * Returns a list of chapters the player can access.
     */
    pub fn get_chapters(&self) -> HashSet<String> {
        let mut chapters = HashSet::new();
        if self.questbook.chapters.is_empty() {
            return chapters;
        }

        for (id, chapter) in self.questbook.chapters.iter() {
            if let Some(req) = &chapter.requirements {
                if self.check_requirements(&req) {
                    chapters.insert(id.clone());
                }
            } else {
                chapters.insert(id.clone());
            }
        }
        chapters
    }

    /**
     * Get the latest scene a player can access.
     */
    pub fn get_scene(&mut self) -> Option<String> {
        match self.get_scenes() {
            Some(mut scenes) => scenes.pop(),
            None => None,
        }
    }

    /**
     * Get the available scenes a player can access.
     */
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
                Requirement::CounterEqual(val, count) => {
                    match self.counters.get(val) {
                        Some(value) => value == count,
                        None => false,
                    }
                },
                Requirement::CounterLessThan(val, count) => {
                    match self.counters.get(val) {
                        Some(value) => value < count,
                        None => false,
                    }
                },
                Requirement::CounterGreaterThan(val, count) => {
                    match self.counters.get(val) {
                        Some(value) => value > count,
                        None => false,
                    }
                },
            }
        )
    }
}