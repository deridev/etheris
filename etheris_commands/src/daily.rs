use crate::prelude::*;

#[command("Pegue um benefício de poder descansar seu personagem a cada 12 horas!")]
#[name("daily")]
#[character_required(true)]
pub async fn daily(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    verify_user_cooldown!(ctx, author, "DAILY");
    ctx.db()
        .cooldowns()
        .create_cooldown(author.id, "DAILY", chrono::Duration::try_hours(12).unwrap())
        .await?;

    let mut character = parse_user_character!(ctx, author);
    character.action_points = character.max_action_points.max(character.action_points);
    character.stats.resistance.value = character.stats.resistance.max;
    character.stats.vitality.value = character.stats.vitality.max;
    character.stats.ether.value = character.stats.ether.max;

    character.add_item(items::special::INTERNAL_KEY, 1, None);

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!("você pegou seu daily e regenerou tudo do seu personagem ao máximo, além de receber **{} 1x {}**!", items::special::INTERNAL_KEY.emoji, items::special::INTERNAL_KEY.display_name),
        )
        .add_emoji_prefix(emojis::SUCCESS),
    )
    .await?;

    Ok(())
}
