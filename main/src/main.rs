mod command_handler;
mod event_handler;

use std::{sync::Arc, time::Duration};

pub use event_handler::EventHandler;

use etheris_common::config;
use etheris_database::{common::DatabaseDateTime, DatabaseState, EtherisDatabase};
use etheris_discord::{
    twilight_gateway::{
        stream::{self, ShardEventStream},
        Config, Intents,
    },
    twilight_model::id::{marker::UserMarker, Id},
};

use etheris_framework::{watcher::Watcher, EtherisClient};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    
    let discord_token = std::env::var(if config::DEBUG {
        "DEBUG_DISCORD_TOKEN"
    } else {
        "DISCORD_TOKEN"
    })
    .expect("expected a valid Discord token");

    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
    let config = Config::new(discord_token.clone(), intents);

    let database = Arc::new(
        EtherisDatabase::new(if config::DEBUG {
            DatabaseState::Debug
        } else {
            DatabaseState::Release
        })
        .await,
    );

    database.setup().await;

    let client = Arc::new(EtherisClient::new(discord_token));
    let watcher = Arc::new(Watcher::new());

    let db = database.clone();
    let client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(15)).await;
            let characters = db.characters().get_all_refilled_characters().await.unwrap();

            for mut character in characters {
                let user_id = Id::new(character.user_id.parse::<u64>().unwrap_or(12345678));

                character.last_refill = DatabaseDateTime::now();
                character.stats.resistance.value = character.stats.resistance.max;
                character.stats.vitality.value = character.stats.vitality.max;
                character.stats.ether.value = character.stats.ether.max;
                if character.settings.is_notifications_enabled
                    && character.action_points != character.max_action_points
                {
                    send_notification(&client_clone, user_id).await.ok();
                }
                character.action_points = character.max_action_points;
                db.characters().save(character.clone()).await.ok();
            }
        }
    });

    // Load a single shard
    let mut shards =
        stream::create_range(0..1, 1, config, |_, builder| builder.build()).collect::<Vec<_>>();

    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((_shard, event)) = stream.next().await {
        let event = match event {
            std::result::Result::Ok(event) => event,
            Err(source) => {
                if source.is_fatal() {
                    eprintln!("FATAL ERROR: {:?}", source);
                    break;
                }

                continue;
            }
        };

        let event_handler = EventHandler::new(client.clone(), watcher.clone(), database.clone());
        tokio::spawn(event_handler.handle(event));
    }
}

async fn send_notification(
    client: &Arc<EtherisClient>,
    user_id: Id<UserMarker>,
) -> anyhow::Result<()> {
    let dm_channel = client
        .http
        .create_private_channel(user_id)
        .await?
        .model()
        .await?;
    client
        .http
        .create_message(dm_channel.id)
        .content("Seus pontos de ação recarregaram e seu personagem descansou! Agora você pode **explorar**, **estudar**, **treinar** e muito mais novamente.")?
        .await?;

    Ok(())
}
