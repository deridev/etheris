use etheris_data::world::regions::RegionKind;

use crate::prelude::*;

#[command("Descanse e alcance seu potencial máximo outra vez!")]
#[name("descansar")]
#[character_required(true)]
pub async fn rest(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    let ap_price = if character.region.kind() == RegionKind::City {
        0
    } else {
        5
    };

    if ap_price != 0 && character.action_points < ap_price {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa de 5 pontos de ação para descansar! Use **/perfil** para ver quando seus pontos de ações recarregam."
            )
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    verify_user_cooldown!(ctx, author, "REST");
    ctx.db()
        .cooldowns()
        .create_cooldown(author.id, "REST", chrono::Duration::minutes(10))
        .await?;

    let mut character = parse_user_character!(ctx, author);
    character.action_points = character.action_points.saturating_sub(ap_price);
    character.stats.resistance.value = character.stats.resistance.max;
    character.stats.vitality.value = character.stats.vitality.max;
    character.stats.ether.value = character.stats.ether.max;

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            "você descansou e retornou ao seu potencial máximo!",
        )
        .add_emoji_prefix(emojis::STAMINA),
    )
    .await?;

    Ok(())
}
