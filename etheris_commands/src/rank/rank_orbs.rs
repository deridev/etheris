use crate::prelude::*;

pub const MEDAL_EMOJIS: &[&str] = &["🥇", "🥈", "🥉"];
pub const DEFAULT_MEDAL: &str = "🏅";

#[command("Lista dos mais ricos da sua região!")]
#[name("rank orbs")]
#[character_required(true)]
pub async fn rank_orbs(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    ctx.reply("Carregando rank...").await?;

    let rank = ctx
        .db()
        .characters()
        .get_orbs_ranking(character.region)
        .await?;

    let embed = EmbedBuilder::new_common()
        .set_color(Color::BLUE)
        .set_title(format!(
            "{} Ranking de orbs de {}",
            emojis::ORB,
            character.region
        ));

    let mut description = String::new();
    for (index, character) in rank.into_iter().enumerate() {
        let user_id = Id::new(character.user_id.parse::<u64>()?);
        let user = ctx.client.get_user(user_id).await?;

        let medal = *MEDAL_EMOJIS.get(index).unwrap_or(&DEFAULT_MEDAL);

        description.push_str(&format!(
            "{medal} **{}**: `{} ◎` ({})",
            character.name,
            character.orbs,
            user.display_name()
        ));

        if user_id == author.id {
            description.push_str(" 🙋");
        }

        description.push('\n');
        if index == 2 {
            description.push('\n');
        }
    }

    ctx.update_interaction_reply(
        Response::from(author.mention()).add_embed(embed.set_description(description)),
    )
    .await?;

    Ok(())
}
