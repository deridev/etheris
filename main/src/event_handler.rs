use std::sync::{atomic::Ordering, Arc};

use etheris_common::config;
use etheris_data::emojis;
use etheris_database::EtherisDatabase;
use etheris_discord::{
    twilight_gateway::Event,
    twilight_model::gateway::payload::incoming::{InteractionCreate, MessageCreate, Ready},
    InteractionType, UserExtension,
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
                if let Err(e) = self.interaction_create(interaction_create).await {
                    eprintln!("Error on interaction create:\n{}\n", e);
                }
            }
            Event::MessageCreate(message_create) => {
                if let Err(e) = self.message_create(message_create).await {
                    eprintln!("Error on message create:\n{}\n", e);
                }
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

    pub async fn message_create(self, message: Box<MessageCreate>) -> anyhow::Result<()> {
        if message.author.bot {
            return Ok(());
        }

        let pings = vec![
            format!("<@!{}>", config::BOT_ID),
            format!("<@{}>", config::BOT_ID),
        ];

        for ping in pings {
            if message.content.trim().to_lowercase() == ping {
                self.client.http.create_message(message.channel_id)
                    .content(&format!("{} **|** Olá, {}! Eu sou Etheris, um bot de RPG único no Discord. Para começar sua jornada, use **/registrar**! Se não souber o que fazer, use **/tutorial** para ser guiado em direção ao mundo de Etheris.", emojis::ETHER, message.author.mention()))
                    ?.await?;
                break;
            }
        }
        Ok(())
    }

    pub async fn interaction_create(
        self,
        interaction: Box<InteractionCreate>,
    ) -> anyhow::Result<()> {
        if interaction.kind == InteractionType::ApplicationCommand {
            command_handler::execute_command(interaction, self.client, self.watcher, self.database)
                .await?;
        }

        Ok(())
    }
}
