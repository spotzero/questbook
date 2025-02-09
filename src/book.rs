use std::collections::HashMap;
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Questbook {
    pub story: Story,
    pub chapters: HashMap<String, Chapter>,
    pub scenes: HashMap<String, Scene>,
    pub items: HashMap<String, Item>,
    pub decisions: HashMap<String, Decision>,
    pub characters: HashMap<String, Character>,
    pub consequences: HashMap<String, Consequence>,
    pub statuses: HashMap<String, Status>,
    pub counters: HashMap<String, Counter>,
    pub triggers: HashMap<String, Trigger>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    /// Title of the story.
    pub title: String,
    /// IDs of the chapters in order.
    pub chapters: Vec<String>,
    /// IDs of decisions that can be made at any time.
    pub decisions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    /// Title of the chapter.
    pub title: String,
    /// IDs of the scenes in this chapter.
    pub scenes: Vec<String>,
    /// Requirements to access this chapter.
    pub requirements: Option<Vec<Requirement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    /// Name of the scene.
    pub name: String,
    /// Background text for the scene.
    pub background: String,
    /// Exposition text for the scene.
    pub exposition: Option<Vec<Exposition>>,
    /// IDs of the characters in this scene.
    pub characters: Option<Vec<String>>,
    /// IDs of the decisions that can be made in this scene.
    pub decisions: Vec<String>,
    /// Requirements to access this scene.
    pub requirements: Option<Vec<Requirement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// Name of the item.
    pub name: String,
    /// Description of the item.
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision {
    /// Description of the decision.
    pub description: Option<String>,
    /// The text for making the decision.
    pub decision: String,
    /// IDs of the consequences of this decision.
    pub consequences: Vec<String>,
    /// Requirements to access this decision.
    pub requirements: Option<Vec<Requirement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Requirement {
    Require(String),
    Refuse(String),
    CounterEqual(String, i32),
    CounterLessThan(String, i32),
    CounterGreaterThan(String, i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consequence {
    /// Description of the action taken.
    pub conclusion: String,
    /// Description of the consequence.
    pub description: Option<String>,
    /// Scene to move to.
    pub scene: Option<String>,
    /// IDs of the items, statuses, or tags to apply.
    pub provides: Option<Vec<String>>, // Items, statuses or tags to apply.
    /// IDs of the items, or statuses to remove.
    pub costs: Option<Vec<String>>, // Items, statuses to remove.
    /// ID of the counters to update and amount to update it.
    pub counter: Option<(String, i32)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    /// Description of the status.
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub inventory: Vec<String>,
    pub states: HashMap<String, CharacterState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterState {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exposition {
    pub requirements: Option<Vec<Requirement>>,
    pub text: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub speaker: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub name: String,
    pub value: i32,
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
    pub description: String,
    pub requirements: Vec<Requirement>,
    pub consequences: Vec<String>,
}

pub fn load_questbook(bookfile: &str) -> Questbook {
    let file = File::open(bookfile).unwrap();
    let questbook: Questbook = serde_yaml::from_reader(file).unwrap();
    questbook
}

impl Questbook {
    pub fn title(&self) -> &String {
        &self.story.title
    }

    pub fn get_consequences_from_decision(&self, decision: &str) -> Vec<String> {
        let mut consequences = Vec::new();
        if let Some(decision) = self.decisions.get(decision) {
            for i in decision.consequences.iter() {
                consequences.push(i.clone());
            }
        }
        consequences
    }

}

pub trait Describe {
    fn describe(&self) -> Vec<String>;
}

impl Describe for Decision {
    fn describe(&self) -> Vec<String> {
        let mut description = Vec::new();
        if self.description.is_some() {
            description.push(self.description.clone().unwrap());
        }
        description
    }
}

impl Describe for Consequence {
    fn describe(&self) -> Vec<String> {
        let mut description = Vec::new();
        description.push(self.conclusion.clone());
        if let Some(desc) = &self.description {
            description.push(desc.clone());
        }
        description
    }
}