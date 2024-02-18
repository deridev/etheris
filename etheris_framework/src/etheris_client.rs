use etheris_database::EtherisDatabase;
use etheris_discord::{
    twilight_model::{
        id::{marker::UserMarker, Id},
        user::{CurrentUser, User},
    },
    DiscordHttpClient,
};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::RwLock;

pub struct EtherisClient {
    pub http: Arc<DiscordHttpClient>,
    pub is_ready: AtomicBool,
    pub users_fighting: RwLock<HashSet<Id<UserMarker>>>,
}

impl EtherisClient {
    pub fn new(token: String) -> Self {
        Self {
            http: Arc::new(DiscordHttpClient::new(token)),
            is_ready: AtomicBool::new(false),
            users_fighting: RwLock::new(HashSet::new()),
        }
    }

    pub async fn current_user(&self) -> anyhow::Result<CurrentUser> {
        Ok(self.http.current_user().await?.model().await?)
    }

    pub async fn get_user(&self, id: Id<UserMarker>) -> anyhow::Result<User> {
        Ok(self.http.user(id).await?.model().await?)
    }

    pub async fn init(&self, _db: Arc<EtherisDatabase>) -> anyhow::Result<()> {
        self.is_ready.swap(true, Ordering::Relaxed);
        let current_user = self.current_user().await?;

        println!("-> Client initialized. Username: {}", current_user.name);

        Ok(())
    }

    pub async fn mark_user_as_fighter(&self, id: Id<UserMarker>) {
        self.users_fighting.write().await.insert(id);
    }

    pub async fn remove_user_fighting_mark(&self, id: Id<UserMarker>) {
        self.users_fighting.write().await.remove(&id);
    }

    pub async fn is_user_fighting(&self, id: Id<UserMarker>) -> bool {
        self.users_fighting.read().await.contains(&id)
    }
}
