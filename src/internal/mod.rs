mod database;

use mongodb::{Client, Database};

use self::database::DatabaseManager;

#[derive(Debug, Clone)]
pub struct InternalManager {
    database: Client,
}

impl InternalManager {
    pub async fn new() -> Self {
        dbg!("Internal Management Initialization!");
        let database = DatabaseManager::new().await;
        Self {
            database: database.client,
        }
    }

    pub fn cache() -> redis::Client {
        todo!()
    }


    pub fn database(&self, name: impl Into<String>) -> Database {
        self.database.database(&name.into()).clone()
    }
}