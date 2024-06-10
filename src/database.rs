use rusqlite::Connection;

pub struct Database {
    pub conn:Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path).unwrap();
        Ok(Self {conn})
    }

    pub fn create_tables(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute("CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY, name TEXT NOT NULL, date TEXT NOT NULL, time TEXT NOT NULL, priority TEXT NOT NULL 
        )",[], )?;

        Ok(())
    }
}