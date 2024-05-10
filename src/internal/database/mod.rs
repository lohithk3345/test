use mongodb::{options::ClientOptions, Client};

pub struct DatabaseManager {
    pub client: Client,
}

impl DatabaseManager {
    pub async fn new() -> Self {
        dbg!("Database Initialization!");
        let options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .map_err(|err| panic!("Error connecting to mongodb! {:#}", err))
            .unwrap();
        Self {
            client: Client::with_options(options).unwrap(),
        }
    }

    pub fn client(&self) -> Client {
        self.client.clone()
    }
}
