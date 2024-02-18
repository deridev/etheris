use std::hash::Hash;

use crate::common::_default_now;
use bson::{doc, oid::ObjectId, Document};
use etheris_common::{clear_string, Cache};
use etheris_data::building::*;
use mongodb::{Collection, Cursor};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use crate::{common::*, EtherisDatabase};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BuildingModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub city_id: ObjectId,
    pub character_owner: ObjectId,
    pub name_id: String,
    pub name: String,
    pub orbs: i64,
    pub kind: BuildingKind,
    pub data: BuildingData,
    // Dates
    #[serde(default = "_default_now")]
    pub created_at: DatabaseDateTime,
}

impl BuildingModel {
    pub fn new(
        city_id: ObjectId,
        character_id: ObjectId,
        orbs: i64,
        name: String,
        data: BuildingData,
    ) -> Self {
        Self {
            id: ObjectId::new(),
            city_id,
            character_owner: character_id,
            name_id: clear_string(&name),
            name,
            orbs,
            kind: data.kind(),
            data,
            created_at: _default_now(),
        }
    }

    pub fn get_job_agency_data(self) -> Option<JobAgencyData> {
        match &self.data.kind() {
            BuildingKind::JobAgency => {
                if let BuildingData::JobAgency(data) = self.data {
                    Some(data)
                } else {
                    panic!("Etheris Implementation Error: `get_job_agency_data` returned invalid data for it's corresponding type.")
                }
            }
            _ => None,
        }
    }

    pub fn get_shop_data(self) -> Option<ShopBuildingData> {
        match &self.data.kind() {
            BuildingKind::Shop => {
                if let BuildingData::Shop(data) = self.data {
                    Some(data)
                } else {
                    panic!("Etheris Implementation Error: `get_shop_data` returned invalid data for it's corresponding type.")
                }
            }
            _ => None,
        }
    }
}

static CACHE_ID: Lazy<Cache<ObjectId, BuildingModel>> = Lazy::new(|| Cache::new(1000));

#[allow(unused)]
pub struct BuildingCommands {
    collection: Collection<BuildingModel>,
    db: EtherisDatabase,
}

impl BuildingCommands {
    pub const fn new(collection: Collection<BuildingModel>, db: EtherisDatabase) -> Self {
        Self { collection, db }
    }

    pub async fn save(&self, building: BuildingModel) -> anyhow::Result<()> {
        CACHE_ID.remove(&building.id);
        self.collection
            .replace_one(query_by_id(building.id), &building, None)
            .await?;
        Ok(())
    }

    async fn get<K: Eq + Hash>(
        &self,
        cache: &Cache<K, BuildingModel>,
        key: K,
        query: Document,
    ) -> anyhow::Result<Option<BuildingModel>> {
        let cached = cache.get_cloned(&key);
        match cached {
            Some(model) => Ok(Some(model)),
            None => {
                let Some(model) = self.collection.find_one(query, None).await? else {
                    return Ok(None);
                };

                cache.insert(key, model.clone());
                Ok(Some(model))
            }
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> anyhow::Result<Option<BuildingModel>> {
        self.get(&CACHE_ID, id, query_by_id(id)).await
    }

    pub async fn get_all_by_city_id(
        &self,
        city_id: ObjectId,
    ) -> anyhow::Result<Cursor<BuildingModel>> {
        let query = doc! {
            "city_id": city_id
        };

        Ok(self.collection.find(query, None).await?)
    }

    pub async fn get_all_by_city_id_as_vec(
        &self,
        city_id: ObjectId,
    ) -> anyhow::Result<Vec<BuildingModel>> {
        let all = self.get_all_by_city_id(city_id).await?;
        Ok(all.collect::<Result<Vec<_>, _>>().await?)
    }

    pub async fn create_building(&self, building: BuildingModel) -> anyhow::Result<()> {
        self.collection.insert_one(building, None).await?;

        Ok(())
    }
}
