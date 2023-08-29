// Database is a trait with two methods, read_db and write_db. For simplicity we will read/write the entire state of the database. JSONFileDatabase is a Struct that implements the Database

trait Database {
    fn read_db();
    fn write_db();
}

// JSONFileDatabase is a Struct that implements the Database
struct JSONFileDatabase {}

