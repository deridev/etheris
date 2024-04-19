use std::hash::Hash;

use bson::{doc, oid::ObjectId, Document};
use etheris_common::{calculate_power_level, Cache};
use etheris_data::{
    items, personality::Personality, world::regions::WorldRegion, ItemValue, ItemValues, SkillKind,
};
use etheris_discord::twilight_model::id::{marker::UserMarker, Id};
use etheris_util::generate_random_character_appearance;
use mongodb::Collection;
use once_cell::sync::Lazy;
use tokio_stream::StreamExt;

use crate::{character_model::CharacterModel, common::*, EtherisDatabase};

static CACHE_ID: Lazy<Cache<ObjectId, CharacterModel>> = Lazy::new(|| Cache::new(1000));
static CACHE_USER_ID: Lazy<Cache<String, CharacterModel>> = Lazy::new(|| Cache::new(1000));

#[allow(unused)]
pub struct CharacterCommands {
    pub collection: Collection<CharacterModel>,
    db: EtherisDatabase,
}

impl CharacterCommands {
    pub const fn new(collection: Collection<CharacterModel>, db: EtherisDatabase) -> Self {
        Self { collection, db }
    }

    pub async fn save(&self, mut character: CharacterModel) -> anyhow::Result<()> {
        let weighted_skills = {
            let mut weight = 0.0;
            for skill in character.skills.iter() {
                let cost = skill.knowledge_cost();
                weight += (cost as f64) / 0.2;
            }

            weight / 5.0
        };

        character.pl = calculate_power_level(
            character.stats.vitality.into(),
            character.stats.resistance.into(),
            character.stats.ether.into(),
            character.stats.strength_level,
            character.stats.intelligence_level,
            character.potential,
            weighted_skills,
        );

        CACHE_ID.remove(&character.id);
        CACHE_USER_ID.remove(&character.user_id);
        self.collection
            .replace_one(query_by_id(character.id), &character, None)
            .await?;
        Ok(())
    }

    pub fn remove_from_cache(&self, character: &CharacterModel) {
        CACHE_ID.remove(&character.id);
        CACHE_USER_ID.remove(&character.user_id);
    }

    async fn get<K: Eq + Hash>(
        &self,
        cache: &Cache<K, CharacterModel>,
        key: K,
        query: Document,
    ) -> anyhow::Result<Option<CharacterModel>> {
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

    pub async fn get_by_id(&self, id: ObjectId) -> anyhow::Result<Option<CharacterModel>> {
        self.get(&CACHE_ID, id, query_by_id(id)).await
    }

    pub async fn get_by_user(&self, user_id: &str) -> anyhow::Result<Option<CharacterModel>> {
        let query = doc! {
            "user_id": user_id,
            "alive": true
        };

        self.get(&CACHE_USER_ID, user_id.to_string(), query).await
    }

    pub async fn get_all_refilled_characters(&self) -> anyhow::Result<Vec<CharacterModel>> {
        let now = DatabaseDateTime::now();
        let date = bson::DateTime::from_chrono(now.0);

        let cursor = self.collection.find(doc! {
            "alive": true,
            "$expr": {
                "$lt": [
                    { "$add": ["$last_refill", { "$multiply": ["$refill_minutes", 60 * 1000] }] },
                    date
                ]
            }
        }, None).await?;
        let vec = cursor.collect::<Result<Vec<_>, _>>().await?;

        Ok(vec)
    }

    pub async fn is_user_registered(&self, user_id: Id<UserMarker>) -> anyhow::Result<bool> {
        if let Some(_character) = self.get_by_user(&user_id.to_string()).await? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_orbs_ranking(
        &self,
        region: WorldRegion,
    ) -> anyhow::Result<Vec<CharacterModel>> {
        let mut cursor = self
            .collection
            .find(
                doc! { "region": bson::to_bson(&region)? },
                mongodb::options::FindOptions::builder()
                    .sort(doc! { "orbs": -1 })
                    .limit(10)
                    .build(),
            )
            .await?;

        let mut models = vec![];
        while cursor.advance().await? {
            models.push(cursor.deserialize_current()?);
        }

        Ok(models)
    }

    pub async fn get_pl_ranking(&self, region: WorldRegion) -> anyhow::Result<Vec<CharacterModel>> {
        let mut cursor = self
            .collection
            .find(
                doc! { "region": bson::to_bson(&region)? },
                mongodb::options::FindOptions::builder()
                    .sort(doc! { "pl": -1 })
                    .limit(10)
                    .build(),
            )
            .await?;

        let mut models = vec![];
        while cursor.advance().await? {
            models.push(cursor.deserialize_current()?);
        }

        Ok(models)
    }

    pub async fn register_character(
        &self,
        user_id: Id<UserMarker>,
        name: String,
        personalities: Vec<Personality>,
        skills: Vec<SkillKind>,
    ) -> anyhow::Result<CharacterModel> {
        let appearance = generate_random_character_appearance();
        let mut character = CharacterModel::new(user_id, name, personalities, vec![], appearance);
        for skill in skills {
            character.aknowledge_skill(skill);
        }

        character.add_item(
            items::special::RECIPE_BOOK,
            1,
            Some(ItemValues {
                values: vec![
                    ItemValue::AlternativeName(format!("Livro de Receitas de {}", character.name)),
                    ItemValue::Recipes(vec![]),
                ],
            }),
        );

        character.add_item(items::consumable::SLICE_OF_BREAD, 5, None);

        self.collection.insert_one(character.clone(), None).await?;

        Ok(character)
    }
}
