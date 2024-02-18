use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::prelude::*;

#[command("Em uma cidade, trabalhe para conseguir alguns orbs!")]
#[name("trabalhar")]
#[character_required(true)]
pub async fn work(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);
    let region = character.region;

    let Some(city) = character.region.city() else {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa estar em uma cidade para poder trabalhar! Use **/viajar** para trocar de região e buscar uma região do tipo cidade."
            ).add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    };

    let ap_price = city.work_ap_cost;
    if character.action_points < ap_price {
        ctx.reply(
            Response::new_user_reply(
                &author,
                format!("você precisa de {ap_price} pontos de ação para trabalhar! Use **/perfil** para ver quando seus pontos de ações recarregam.")
            ).add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    verify_user_cooldown!(ctx, author, "WORK");
    ctx.db()
        .cooldowns()
        .create_cooldown(author.id, "WORK", chrono::Duration::minutes(5))
        .await?;

    let rng = &mut StdRng::from_entropy();

    let salary = rng.gen_range(city.wage.0..=city.wage.1);
    let strength_xp = rng.gen_range(city.work_strength_xp_gain.0..=city.work_strength_xp_gain.1);
    let health_xp = rng.gen_range(city.work_health_xp_gain.0..=city.work_health_xp_gain.1);
    let intelligence_xp =
        rng.gen_range(city.work_intelligence_xp_gain.0..=city.work_intelligence_xp_gain.1);

    character.action_points -= ap_price;
    character.intelligence_xp += intelligence_xp;
    character.knowledge_xp += intelligence_xp;
    character.strength_xp += strength_xp;
    character.health_xp += health_xp;
    character.add_orbs(salary);

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você trabalhou na cidade **{}** e recebeu **{} ◎** ao custo de **{} pontos de ação**!\nVocê ganhou **{} XP em força**, **{} XP em vida** e **{} XP em inteligência**.",
                region, salary, ap_price,
                strength_xp, health_xp, intelligence_xp
            ),
        )
        .add_emoji_prefix(emojis::ORB),
    )
    .await?;

    Ok(())
}
