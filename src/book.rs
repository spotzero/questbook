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
    pub statuses: HashMap<String, Status>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub title: String,
    pub chapters: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub scenes: Vec<String>,
    pub requirements: Option<Vec<Requirement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub background: String,
    pub exposition: Vec<Exposition>,
    pub characters: Vec<String>,
    pub inventory: Vec<String>,
    pub decisions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision {
    pub decision: String,
    pub consequence: Consequence,
    pub requirements: Vec<Requirement>,
    pub consumes: Option<Vec<String>>, // Items to consume.
    pub removes: Option<Vec<String>>, // Statuses to remove.

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requirement {
    need: Option<Condition>,
    reject: Option<Condition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Condition {
    Item(String),
    Consequence(String),
    Status(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consequence {
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
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
    pub text: Vec<Text>,
    pub requirements: Vec<Requirement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub speaker: Option<String>,
    pub text: String,
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
}