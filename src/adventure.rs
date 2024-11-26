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
        if self.scene.is_some() {
            return self.scene.clone();
        }

        if self.chapter.is_none() {
            return None;
        }

        let scenes = self.get_scenes();
        if scenes.is_empty() {
            return None;
        } else {
            for scene in self.questbook.chapters[self.chapter.as_ref().unwrap()].scenes.iter() {
                if scenes.contains(scene) {
                    return Some(scene.clone());
                }
            }
        }
        return None;
    }

    /**
     * Get the available scenes a player can access.
     */
    pub fn get_scenes(&self) -> HashSet<String> {
        let mut scenes = HashSet::new();

        if self.chapter.is_none() {
            return scenes;
        }

        for (id, scene) in self.questbook.scenes.iter() {
            if self.chapter_contains_scene(self.chapter.as_ref().unwrap(), id) {
                if let Some(req) = &scene.requirements {
                    if self.check_requirements(&req) {
                        scenes.insert(id.clone());
                    }
                } else {
                    scenes.insert(id.clone());
                }
            }
        }
        scenes
    }

    pub fn chapter_contains_scene(&self, chapter: &str, scene: &str) -> bool {
        if let Some(chapter) = self.questbook.chapters.get(chapter) {
            for value in chapter.scenes.iter() {
                if value.eq(scene) {
                    return true;
                }
            }
        }
        false
    }

    /**
     * Get the available decisions a player can make.
     */
    pub fn get_decisions(&self) -> HashSet<String> {
        let mut decisions = HashSet::new();
        if self.chapter.is_none() || self.scene.is_none() {
            return decisions;
        }

        for (id, decision) in self.questbook.decisions.iter() {
            if self.scene_contains_decision(self.scene.as_ref().unwrap(), id) || self.decision_is_global(id) {
                if let Some(req) = &decision.requirements {
                    if self.check_requirements(&req) {
                        decisions.insert(id.clone());
                    }
                } else {
                    decisions.insert(id.clone());
                }
            }
        }
        decisions
    }

    /**
     * Check if a decision is in a scene.
     */
    pub fn scene_contains_decision(&self, scene: &str, decision: &str) -> bool {
        if let Some(scene) = self.questbook.scenes.get(scene) {
            for value in scene.decisions.iter() {
                if value.eq(decision) {
                    return true;
                }
            }
        }
        false
    }

    /**
     * Check if a decision is global.
     */
    pub fn decision_is_global(&self, decision: &str) -> bool {
        if let Some(decisions) = self.questbook.story.decisions.as_ref() {
            for value in decisions {
                if value.eq(decision) {
                    return true;
                }
            }
        }
        false
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