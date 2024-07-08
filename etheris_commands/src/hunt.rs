use etheris_rpg::{data::enemies::get_enemies_by_regions, encounter, FighterData};
use rand::{rngs::StdRng, SeedableRng};

use crate::prelude::*;

#[command("Explore a região visando apenas inimigos para enfrentar!")]
#[name("caçar")]
#[character_required(true)]
pub async fn hunt(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);

    if character.action_points < 1 {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você precisa de **1 ponto de ação** para caçar! Veja quando seus pontos de ações recarregam em **/perfil**.\n❓ **Dica**: estudar (`/estudar`) faz seus pontos de ação aumentarem e recarregarem mais rápido!"
            )
                .add_emoji_prefix(emojis::ERROR)
                .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    verify_user_cooldown!(ctx, author, "HUNT");

    let enemies = get_enemies_by_regions(&[character.region]);
    if enemies.is_empty() {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você não encontrou nenhum inimigo! Tente viajar para outra região para encontrar mais coisas interessantes."
            )
            .add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    ctx.db()
        .cooldowns()
        .create_cooldown(
            author.id,
            "HUNT",
            chrono::Duration::try_seconds(10).unwrap(),
        )
        .await?;

    let enemies = enemies
        .iter()
        .filter_map(|e| {
            if !e.base_probability.generate_random_bool() {
                return None;
            }

            let regions = e.regions.iter().find(|r| r.0 == character.region)?;

            Some((e.clone(), *regions))
        })
        .collect::<Vec<_>>();

    use rand::seq::SliceRandom;

    let Ok((enemy, ..)) =
        enemies.choose_weighted(&mut StdRng::from_entropy(), |(_, (_, prob))| *prob)
    else {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você não encontrou nenhum inimigo! Aproveite a paz.",
            )
            .add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    };

    let mut enemies = vec![enemy.clone()];

    let allies = enemy.allies.clone().unwrap_or_default();
    for ally in allies
        .into_iter()
        .filter(|e| e.0.generate_random_bool())
        .map(|e| e.1)
    {
        enemies.push((*ally).clone());
    }

    let enemies = enemies
        .into_iter()
        .map(|e| {
            let reward =
                e.drop
                    .to_reward(&mut StdRng::from_entropy(), character.pl, e.power_level());
            FighterData::new_from_enemy(1, reward, e)
        })
        .collect::<Vec<_>>();

    let result = encounter::prompt_encounter(&mut ctx, author.clone(), enemies).await?;
    let Some(_result) = result else {
        return Ok(());
    };

    Ok(())
}
