use anyhow::Result;

use crate::db::{Database, JSONFileDatabase};
use crate::models::{DBState, Epic, Status, Story};

pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        JiraDatabase {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut dbstate = self.read_db()?;
        match dbstate.add_epic(epic) {
            Ok(id) => {
                self.database.write_db(&dbstate)?;
                Ok(id)
            }
            Err(err) => Err(err),
        }
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut dbstate = self.read_db()?;
        match dbstate.add_story(story, epic_id) {
            Ok(id) => {
                self.database.write_db(&dbstate)?;
                Ok(id)
            }
            Err(err) => Err(err),
        }
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut dbstate = self.read_db()?;
        match dbstate.delete_epic(epic_id) {
            Ok(()) => {
                self.database.write_db(&dbstate)?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut dbstate = self.read_db()?;
        match dbstate.delete_story(epic_id, story_id) {
            Ok(_) => {
                self.database.write_db(&dbstate)?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut dbstate = self.read_db()?;
        match dbstate.update_epic_status(epic_id, status) {
            Ok(_) => {
                self.database.write_db(&dbstate)?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut dbstate = self.read_db()?;
        match dbstate.update_story_status(story_id, status) {
            Ok(id) => {
                self.database.write_db(&dbstate)?;
                Ok(id)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::MockDB;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        // TODO: fix this error by deriving the appropriate traits for Epic
        let result = db.create_epic(epic.clone());

        assert!(result.is_ok());

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        // TODO: fix this error by deriving the appropriate traits for Story
        let result = db.create_story(story.clone(), epic_id);
        assert!(result.is_ok());

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id));
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert!(result.is_ok());

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();

        let non_existent_epic_id = 999;

        let result = db.delete_story(non_existent_epic_id, story_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let non_existent_story_id = 999;

        let result = db.delete_story(epic_id, non_existent_story_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert!(result.is_ok());

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(
            db_state
                .epics
                .get(&epic_id)
                .unwrap()
                .stories
                .contains(&story_id),
            false
        );
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert!(result.is_err());
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert!(result.is_ok());

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert!(result.is_err());
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert!(result.is_ok());

        let db_state = db.read_db().unwrap();

        assert_eq!(
            db_state.stories.get(&story_id).unwrap().status,
            Status::Closed
        );
    }
}
