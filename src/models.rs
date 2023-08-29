use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

impl DBState {
    pub fn new() -> Self {
        DBState {
            last_item_id: 0,
            epics: HashMap::new(),
            stories: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epic_new_test() {
        let e: Epic = Epic::new(
            "Project 1".to_string(),
            "Start of bootcamp projects".to_string(),
        );
        assert_eq!(e.status, Status::Open);
    }

    #[test]
    fn story_new_test() {
        let s: Story = Story::new(
            "Create database".to_string(),
            "Implment models and database for project".to_string(),
        );
        assert_eq!(s.status, Status::Open);
    }
}
