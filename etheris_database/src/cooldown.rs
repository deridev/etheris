use bson::{doc, oid::ObjectId, Document};
use chrono::Duration;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{common::*, EtherisDatabase};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CooldownModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub identifier: String,
    pub ends_at: DatabaseDateTime,
}

impl CooldownModel {
    pub fn new(user_id: String, identifier: String, ends_at: Duration) -> Self {
        Self {
            id: ObjectId::new(),
            user_id,
            identifier,
            ends_at: DatabaseDateTime(DatabaseDateTime::now().0 + ends_at),
        }
    }
}

#[allow(unused)]
pub struct CooldownCommands {
    collection: Collection<CooldownModel>,
    db: EtherisDatabase,
}

impl CooldownCommands {
    pub const fn new(collection: Collection<CooldownModel>, db: EtherisDatabase) -> Self {
        Self { collection, db }
    }

    pub async fn save(&self, cd: CooldownModel) -> anyhow::Result<()> {
        self.collection
            .replace_one(query_by_id(cd.id), &cd, None)
            .await?;
        Ok(())
    }

    async fn get(&self, query: Document) -> anyhow::Result<Option<CooldownModel>> {
        Ok(self.collection.find_one(query, None).await?)
    }

    pub async fn get_by_id(&self, id: ObjectId) -> anyhow::Result<Option<CooldownModel>> {
        self.get(query_by_id(id)).await
    }

    pub async fn expire_cooldowns(&self) -> anyhow::Result<()> {
        let now = DatabaseDateTime::now();
        let date = bson::DateTime::from_chrono(now.0);

        self.collection
            .delete_many(
                doc! {
                    "ends_at": {
                        "$lt": date
                    }
                },
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn delete_all_user_cooldowns(&self, user_id: String) -> anyhow::Result<()> {
        let query = doc! {
            "user_id": user_id,
        };

        self.collection.delete_many(query, None).await?;
        Ok(())
    }

    pub async fn get_user_cooldown(
        &self,
        user_id: String,
        cd_identifier: String,
    ) -> anyhow::Result<Option<CooldownModel>> {
        self.expire_cooldowns().await?;
        let query = doc! {
            "user_id": user_id,
            "identifier": cd_identifier
        };

        self.get(query).await
    }

    pub async fn create_cooldown(
        &self,
        user_id: impl ToString,
        identifier: impl ToString,
        cooldown: chrono::Duration,
    ) -> anyhow::Result<()> {
        self.expire_cooldowns().await?;
        self.collection
            .insert_one(
                CooldownModel::new(user_id.to_string(), identifier.to_string(), cooldown),
                None,
            )
            .await?;

        Ok(())
    }
}
