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
    pub exposition: Option<Vec<Exposition>>,
    pub characters: Option<Vec<String>>,
    pub decisions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision {
    pub description: String,
    pub decision: String,
    pub consequence: Vec<String>,
    pub requirements: Option<Vec<Requirement>>,
    pub costs: Option<Vec<String>>, // Statuses and items to remove.
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Requirement {
    Request(String),
    Reject(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consequence {
    conclusion: String,
    description: Option<String>,
    provides: Option<Vec<String>>, // Items or statuses to give.
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
    pub requirements: Vec<Requirement>,
    pub text: Vec<Text>,
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