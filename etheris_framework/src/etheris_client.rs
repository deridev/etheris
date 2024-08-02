use etheris_common::{config, Color};
use etheris_database::EtherisDatabase;
use etheris_discord::{
    twilight_model::{
        gateway::payload::incoming::GuildCreate, id::{marker::UserMarker, Id}, user::{CurrentUser, User}
    }, DiscordHttpClient, EmbedAuthor, EmbedBuilder, GuildExtension
};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::RwLock;

#[derive(Debug)]
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

    pub async fn emit_guild_create_hook(
        &self,
        guild_create: Box<GuildCreate>,
    ) -> anyhow::Result<()> {
        if config::DEBUG {
            return Ok(());
        }

        let hook_id = std::env::var("GUILD_HOOK_ID")?.parse::<u64>()?;
        let hook_token = std::env::var("GUILD_HOOK_TOKEN")?;

        let guild = self
            .http
            .guild(guild_create.id)
            .with_counts(true)
            .await?
            .model()
            .await?;
        let member_count = guild
            .approximate_member_count
            .map(|m| m.to_string())
            .unwrap_or(String::from("?"));

        let embed = EmbedBuilder::new_common()
            .set_color(Color::CYAN_GREEN)
            .set_thumbnail(guild.icon_url())
            .set_author(EmbedAuthor {
                name: "Servidor novo!".to_string(),
                icon_url: Some(guild.icon_url()),
            })
            .add_inlined_field("📄 Nome", format!("**{}**", guild.name))
            .add_inlined_field("👥 Membros", format!("**{}**", member_count))
            .add_footer_text(format!(
                "ID do servidor: {}\nID do dono: {}",
                guild.id, guild.owner_id
            ));

        self.http
            .execute_webhook(Id::new(hook_id), &hook_token)
            .embeds(&[embed.build()])?
            .await?;
        Ok(())
    }
}
