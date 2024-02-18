use crate::prelude::*;

#[command("Veja as estatísticas de um personagem em Etheris")]
#[name("estatísticas")]
#[character_required(true)]
pub async fn stats(
    mut ctx: CommandContext,
    #[rename("usuário")]
    #[description("O usuário que você quer ver as estatísticas")]
    user: Option<User>,
) -> anyhow::Result<()> {
    let user = user.unwrap_or(ctx.author().await?);
    let character = parse_user_character!(ctx, user);

    let image = character.create_image_bufer();
    let attachment =
        image.map(|image| DiscordAttachment::from_bytes("image.png".to_owned(), image, 1));

    let embed = EmbedBuilder::new()
        .set_author(EmbedAuthor {
            name: format!("Personagem de {}", user.display_name()),
            icon_url: Some(user.avatar_url()),
        })
        .set_color(Color::YELLOW)
        .set_thumbnail(if let Some(attachment) = &attachment {
            format!("attachment://{}", attachment.filename)
        } else {
            user.avatar_url()
        })
        .add_inlined_field("PvE", render_stats(character.stats.pve))
        .add_inlined_field("PvP", render_stats(character.stats.pvp))
        .set_current_timestamp();

    ctx.reply(
        Response::from(embed).set_attachments(if let Some(attachment) = attachment {
            vec![attachment]
        } else {
            vec![]
        }),
    )
    .await?;

    Ok(())
}

fn render_stats(stats: BattleStats) -> String {
    let win_rate =
        ((stats.wins as f32) / ((stats.losses + stats.wins) as f32) * 100.0).round() as i32;
    format!("**Vitórias**: `{}`\n**Derrotas**: `{}`\n**Porcentagem de vitórias**: {win_rate}%\n**Desistências**: `{}`\n**Riscos de Vida**: `{}`\n\n**Nocautes**: {}\n**Assassinatos**: {}",
        stats.wins, stats.losses, stats.withdrawals, stats.life_risks, stats.knockouts, stats.kills
    )
}
