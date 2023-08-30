use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

    pub fn add_epic(&mut self, epic: Epic) -> Result<u32> {
        let id: u32 = self.last_item_id + 1;
        self.last_item_id = id;
        self.epics.insert(id, epic);
        Ok(id)
    }

    pub fn update_epic_status(&mut self, epic_id: u32, status: Status) -> Result<()> {
        if let Some(epic) = self.epics.get_mut(&epic_id) {
            epic.status = status;
            Ok(())
        } else {
            Err(anyhow::anyhow!(format!("No Epic with id {epic_id}")))
        }
    }

    pub fn delete_epic(&mut self, epic_id: u32) -> Result<()> {
        // Remove the Epic
        match self.epics.remove(&epic_id) {
            Some(epic) => {
                // Remove all the Stories owned by the Epic
                for story_id in epic.stories {
                    self.stories.remove(&story_id);
                }
                Ok(())
            }
            None => Err(anyhow::anyhow!("No Epic with id {epic_id}")),
        }
    }

    pub fn add_story(&mut self, story: Story, epic_id: u32) -> Result<u32> {
        // Get the Epic
        let epic: &mut Epic = self
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow::anyhow!(format!("No Epic with id {epic_id}")))?;

        // Add the Story
        let id: u32 = self.last_item_id + 1;
        self.last_item_id = id;
        self.stories.insert(id, story);

        // Add the Story to the Epic
        epic.stories.push(id);
        Ok(id)
    }

    pub fn update_story_status(&mut self, story_id: u32, status: Status) -> Result<()> {
        let stories = &mut self.stories;
        if let Some(story) = stories.get_mut(&story_id) {
            story.status = status;
            Ok(())
        } else {
            Err(anyhow::anyhow!(format!("No Story with id {story_id}")))
        }
    }

    pub fn delete_story(&mut self, epic_id: u32, story_id: u32) -> Result<()> {
        match self.stories.remove(&story_id) {
            Some(_) => {
                let epics = &mut self.epics;
                if let Some(epic) = epics.get_mut(&epic_id) {
                    let values = &mut epic.stories;
                    values.retain(|x| *x != story_id);
                    Ok(())
                } else {
                    Err(anyhow::anyhow!(format!("No Epic with id {epic_id}")))
                }
            }
            None => Err(anyhow::anyhow!(format!("No Story with id {story_id}"))),
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
