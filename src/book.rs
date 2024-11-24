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
    pub requirements: Vec<Requirement>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Requirement {
    Item(String),
    Consequence(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consequence {
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub inventory: Vec<String>,
}

pub fn load_questbook(bookfile: &str) -> Questbook {
    let file = File::open(bookfile).unwrap();
    let questbook: Questbook = serde_yaml::from_reader(file).unwrap();
    questbook
}