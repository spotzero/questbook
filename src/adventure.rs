use crate::book::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::Extend;
use std::ops::Deref;

pub struct Adventure {
    pub questbook: Questbook,
    pub tags: HashSet<String>,
    pub inventory: HashSet<String>,
    pub statuses: HashSet<String>,
    pub chapter: Option<String>,
    pub scene: Option<String>,
    pub counters: HashMap<String, i32>,
    pub state: AdventureState,
    pub log: Vec<String>,
}
/*
pub struct Changes {
    chapter: Strung,
    scene,
    decision,
    counter,
    item,
    status,
    tag,
}*/

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
            log: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.chapter = self.get_chapter();
        match self.chapter {
            Some(_) => {
                self.state = AdventureState::Started;
                self.scene = self.get_scene();
                self.log.push(format!("Adventure started - Chapter: {} - Scene: {}", self.chapter.as_ref().unwrap_or(&"None".to_string()), self.scene.as_ref().unwrap_or(&"None".to_string())));
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
            if self.check_requirements_options(&chapter.requirements) {
                chapters.insert(id.clone());
            }
        }
        chapters
    }

    pub fn change_chapter(&mut self) {
        match self.get_chapter() {
            Some(chapter) => {
                if self.chapter.eq(&Some(chapter.clone())) {
                    return;
                }
                self.chapter = Some(chapter.clone());
                self.log.push(format!("Chapter changed to: {}", chapter));
                self.change_scenes();
            },
            None => {
                self.chapter = None;
                self.log.push("No more chapters available".to_string());
                self.state = AdventureState::Ended;
            },
        }
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
                if self.check_requirements_options(&scene.requirements) {
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
                if self.check_requirements_options(&decision.requirements) {
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

    pub fn change_scenes(&mut self) {
        let scenes = self.get_scenes();
        if scenes.is_empty() {
            self.scene = None;
            self.log.push("No more scenes available".to_string());
            self.state = AdventureState::Ended;
            return;
        }
        self.change_scene(scenes.iter().next().unwrap(), false);
    }

    pub fn change_scene(&mut self, scene: &str, check_triggers: bool) {
        if !self.get_scenes().contains(scene) {
            return;
        }
        self.scene = Some(scene.to_string());
        self.log.push(format!("Scene changed to: {}", scene));
        if check_triggers {
            self.check_triggers();
        }
    }

    /**
     * Make a decision and apply consequences.
     */
    pub fn make_decision(&mut self, decision: &str) -> Vec<String> {
        let mut output = Vec::new();
        if !self.get_decisions().contains(decision) {
            return output;
        }
        self.log.push(format!("Decision made: {}", decision));
        output.extend(self.questbook.decisions.get(decision).unwrap().describe());
        for consequence in self.questbook.get_consequences_from_decision(decision) {
            self.apply_consequence(&consequence);
        }
        self.check_triggers();
        output
    }

    /**
     * Check all triggers to see if any are met.
     */
    fn check_triggers(&mut self) {
        let mut consequences = Vec::new();
        for (id, trigger) in self.questbook.triggers.iter() {
            if self.check_requirements(&trigger.requirements) {
                consequences.push(id.clone());
            }
        }
        if consequences.is_empty() {
            return;
        }
        for consequence in consequences {
            self.apply_consequence(&consequence);
        }
    }

    /**
     * Apply a consequence.
     */
    fn apply_consequence(&mut self, consequence: &str) -> Vec<String> {
        self.log.push(format!("Applying consequence: {}", consequence));
        let mut output = Vec::new();

        // Apply consequences.
        let consequence = self.questbook.consequences.get(consequence).unwrap();

        // Provide items, statuses, or tags.
        if let Some(provides) = &consequence.provides {
            for provide in provides {
                if self.questbook.items.contains_key(provide) {
                    self.inventory.insert(provide.clone());
                    self.log.push(format!("Item {} added", provide));
                } else if self.questbook.statuses.contains_key(provide) {
                    self.statuses.insert(provide.clone());
                    self.log.push(format!("Status {} added", provide));
                } else {
                    self.tags.insert(provide.clone());
                    self.log.push(format!("Tag {} added", provide));
                }
            }
        }

        // Remove items, statuses, or tags.
        if let Some(costs) = &consequence.costs {
            for cost in costs {
                if self.questbook.items.contains_key(cost) {
                    self.inventory.remove(cost);
                    self.log.push(format!("Item {} removed", cost));
                } else {
                    self.statuses.remove(cost);
                    self.log.push(format!("Status {} removed", cost));
                }
            }
        }

        // Change counters.
        if let Some((counter, value)) = &consequence.counter {
            if let Some(counter_value) = self.counters.get_mut(counter) {
                *counter_value += value;
                self.log.push(format!("Counter {} changed to: {}", counter, counter_value));
            }
        }

        if let Some(scene) = &consequence.scene.clone() {
            self.change_scene(scene, false);
        }

        self.change_chapter();
        output
    }

    /**
     * Check if requirements are met of in an option.
     */
    fn check_requirements_options(&self, req_opt: &Option<Vec<Requirement>>) -> bool {
        match req_opt {
            None => true,
            Some(req) => self.check_requirements(req),
        }
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