pub mod building;
pub mod character_commands;
pub mod character_model;
pub mod common;
pub mod cooldown;

use std::sync::Arc;

use bson::doc;
use character_commands::CharacterCommands;

use character_model::CharacterModel;
use cooldown::{CooldownCommands, CooldownModel};
use mongodb::{Client, Collection, Database, IndexModel};

pub use mongodb::bson;
pub use mongodb::error::Error as MongoDBError;

#[derive(Debug, Clone)]
pub enum DatabaseState {
    Debug,
    Release,
}

#[derive(Debug, Clone)]
pub struct EtherisDatabase {
    /* MongoDB's Client uses Arc internally */
    client: Client,
    state: Arc<DatabaseState>,
}

impl EtherisDatabase {
    pub async fn new(state: DatabaseState) -> EtherisDatabase {
        let uri = std::env::var("DATABASE_URI").unwrap();

        let client = Client::with_uri_str(&uri).await.unwrap();

        EtherisDatabase {
            client,
            state: Arc::new(state),
        }
    }

    pub async fn setup(&self) {
        // COOLDOWN INDEXES
        let cooldowns: Collection<CooldownModel> = self.db().collection("cooldowns");
        cooldowns
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "user_id": 1, "identifier": 1 })
                    .build(),
                None,
            )
            .await
            .unwrap();

        // CHARACTERS INDEXES
        let characters: Collection<CharacterModel> = self.db().collection("characters");
        characters
            .create_index(
                IndexModel::builder().keys(doc! { "user_id": 1 }).build(),
                None,
            )
            .await
            .unwrap();
    }

    pub fn db(&self) -> Database {
        self.client.database(match *self.state {
            DatabaseState::Debug => "etheris_debug",
            DatabaseState::Release => "etheris_release",
        })
    }

    pub fn characters(&self) -> CharacterCommands {
        let collection = self.db().collection("characters");
        CharacterCommands::new(collection, self.clone())
    }

    pub fn cooldowns(&self) -> CooldownCommands {
        let collection = self.db().collection("cooldowns");
        CooldownCommands::new(collection, self.clone())
    }
}
