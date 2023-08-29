#[derive(Debug, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<i32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic { name, description, status: Status::Open, stories: vec![] }
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story { name, description, status: Status::Open }
    }
}

pub struct DBState {
    pub last_item_id: i32,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
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

