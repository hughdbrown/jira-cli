use std::fs;

use anyhow::Result;

use crate::models::{DBState, Epic, Status, Story};

// Database is a trait with two methods, read_db and write_db.
// For simplicity we will read/write the entire state of the database.
trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

// JSONFileDatabase is a Struct that implements the Database
struct JSONFileDatabase {
    file_path: String,
}

impl JSONFileDatabase {
    fn new(file_path: String) -> Self {
        JSONFileDatabase { file_path }
    }
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let data = fs::read_to_string(&self.file_path)?;
        if data.is_empty() {
            return Ok(DBState::new());
        }
        let dbstate: DBState = serde_json::from_str(&data)?;
        Ok(dbstate)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let state: String = serde_json::to_string_pretty(&db_state)?;
        fs::write(&self.file_path, state)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase::new("INVALID_PATH".to_owned());
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let path = tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile path to str");
            let db = JSONFileDatabase::new(path.to_string());

            let result = db.read_db();

            assert!(result.is_err());
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let path = tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile path to str");
            let db = JSONFileDatabase::new(path.to_string());

            let result = db.read_db();

            assert!(result.is_ok());
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let path = tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile path to str");
            let db = JSONFileDatabase::new(path.to_string());

            let story = Story {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
            };
            let epic = Epic {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert!(write_result.is_ok());
            assert_eq!(read_result, state);
        }
    }
}
