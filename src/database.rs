use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).unwrap();
        Self { conn }
    }

    pub fn create_tables(&self) {
        self.conn.execute("CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY, name TEXT NOT NULL, date TEXT NOT NULL, time TEXT NOT NULL, priority TEXT NOT NULL 
        )",[], ).unwrap();
    }
}
