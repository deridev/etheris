use crate::prelude::*;

use etheris_rpg::*;

pub const MEDAL_EMOJIS: &[&str] = &["🥇", "🥈", "🥉"];
pub const DEFAULT_MEDAL: &str = "🏅";

#[command("Lista dos mais fortes da sua região!")]
#[name("rank poder")]
#[character_required(true)]
pub async fn rank_pl(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    ctx.reply("Carregando rank...").await?;

    let rank = ctx
        .db()
        .characters()
        .get_pl_ranking(character.region)
        .await?;

    let embed = EmbedBuilder::new_common()
        .set_color(Color::YELLOW)
        .set_title(format!("{} Ranking de poder de {}", "💪", character.region));

    let mut description = String::new();
    for (index, character) in rank.into_iter().enumerate() {
        let user_id = Id::new(character.user_id.parse::<u64>()?);
        let user = ctx.client.get_user(user_id).await?;

        let pl = FighterData::new_from_character(0, &character, user.clone(), Default::default())
            .power_level();

        let medal = *MEDAL_EMOJIS.get(index).unwrap_or(&DEFAULT_MEDAL);

        description.push_str(&format!(
            "{medal} **{}**: `{} PL` ({})",
            character.name,
            pl,
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
