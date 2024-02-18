use std::sync::{atomic::Ordering, Arc};

use etheris_database::EtherisDatabase;
use etheris_discord::{
    twilight_gateway::Event,
    twilight_model::gateway::payload::incoming::{InteractionCreate, Ready},
};
use etheris_framework::{watcher::Watcher, EtherisClient};

use crate::command_handler;

pub struct EventHandler {
    client: Arc<EtherisClient>,
    watcher: Arc<Watcher>,
    database: Arc<EtherisDatabase>,
}

impl EventHandler {
    pub fn new(
        client: Arc<EtherisClient>,
        watcher: Arc<Watcher>,
        database: Arc<EtherisDatabase>,
    ) -> Self {
        Self {
            client,
            watcher,
            database,
        }
    }

    pub async fn handle(self, event: Event) {
        self.watcher.process(&event);

        match event {
            Event::Ready(ready) => {
                if self.client.is_ready.load(Ordering::Relaxed) {
                    return;
                }

                let client = self.client.clone();
                let database = self.database.clone();

                self.ready(ready).await.unwrap();
                client.init(database).await.unwrap();
            }
            Event::InteractionCreate(interaction_create) => {
                self.interaction_create(interaction_create).await.ok();
            }
            _ => {}
        };
    }

    pub async fn ready(self, ready: Box<Ready>) -> anyhow::Result<()> {
        let current_user = self.client.current_user().await?;
        println!("{} is ready!", current_user.name);

        command_handler::register_commands(ready.application.id, self.client).await;

        Ok(())
    }

    pub async fn interaction_create(
        self,
        interaction: Box<InteractionCreate>,
    ) -> anyhow::Result<()> {
        command_handler::execute_command(interaction, self.client, self.watcher, self.database)
            .await
    }
}
