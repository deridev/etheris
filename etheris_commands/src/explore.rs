use etheris_data::items::get_item_by_weapon;
use etheris_rpg::{
    data::{
        enemies::{get_enemy_by_id, ALL_ENEMIES},
        Reward,
    },
    list::get_boxed_skill_from_kind,
    Battle, BattleController, BattleSettings, FighterData,
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::prelude::*;

#[command("Explore sua região em busca de itens, inimigos ou outras coisas!")]
#[name("explorar")]
#[character_required(true)]
pub async fn explore(mut ctx: CommandContext) -> anyhow::Result<()> {
    explore_enemy(ctx).await?;
    Ok(())
}

async fn explore_enemy(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    if ctx.client.is_user_fighting(author.id).await {
        ctx.send(
            Response::new_user_reply(&author, "você já está no meio de uma batalha!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let character = parse_user_character!(ctx, author);

    let enemies = ALL_ENEMIES
        .iter()
        .filter_map(|e| {
            if !e.base_probability.generate_random_bool() {
                return None;
            }

            let regions = e.regions.iter().find(|r| r.0 == character.region)?;

            Some((*e, *regions))
        })
        .collect::<Vec<_>>();

    let Ok((enemy, ..)) =
        enemies.choose_weighted(&mut StdRng::from_entropy(), |(_, (_, prob))| *prob)
    else {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você não encontrou nenhum inimigo na exploração! Aproveite a paz.",
            )
            .add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    };

    let enemy_allies = match enemy.allies {
        None => vec![],
        Some(allies) => allies
            .iter()
            .filter(|a| a.0.generate_random_bool())
            .filter_map(|a| get_enemy_by_id(a.1))
            .collect(),
    };

    let mut drop: Reward = enemy.drop.into(); // <- Important. Implementation details at enemies.rs (impl From<EnemyReward> for Reward)
    for ally in enemy_allies.iter() {
        let enemy_drop: Reward = ally.drop.into();
        drop = drop.add(enemy_drop);
    }

    let character_fighter =
        FighterData::new_from_character(0, &character, author.clone(), Reward::default());
    let enemy_fighter = FighterData::new_from_enemy(1, drop.clone(), *enemy);

    let enemy_allies_fighters = enemy_allies
        .iter()
        .map(|ally| FighterData::new_from_enemy(1, Default::default(), *ally))
        .collect::<Vec<_>>();

    let enemy_power_level = {
        let mut power_level = enemy_fighter.power_level();
        for enemy_ally in &enemy_allies_fighters {
            power_level += enemy_ally.power_level();
        }

        power_level
    };

    let power_diff = (character_fighter.power_level() - enemy_power_level) / 2;

    let color = match power_diff {
        i64::MIN..=-100 => Color::RED,
        -98..=-6 => Color::LIGHT_RED,
        -5..=10 => Color::YELLOW,
        11..=30 => Color::LIGHT_BLUE,
        31..=100 => Color::LIGHT_GREEN,
        _ => Color::GREEN,
    };

    let warning = if character.stats.vitality.value < character.stats.vitality.max / 2 {
        "⚠️ **__Você não está com a vida cheia!__**\nTome cuidado ou descanse antes de explorar.\n"
    } else {
        ""
    };

    let embed = EmbedBuilder::new_common()
        .set_color(color)
        .set_author(EmbedAuthor {
            name: "Você encontrou um inimigo!".into(),
            icon_url: Some(author.avatar_url()),
        })
        .set_description(format!(
            "{}## {}\n{}{}\n{} **{}**\n{} {}\n{} {}\n\n**Habilidades**: `{}`{}",
            warning,
            enemy.name,
            enemy
                .personalities
                .iter()
                .map(|p| bold(&p.to_string()))
                .collect::<Vec<_>>()
                .join(", "),
            if let Some(weapon) = enemy.weapon {
                let weapon = get_item_by_weapon(weapon);
                format!("\n**Arma**: {} {}", weapon.display_name, weapon.emoji)
            } else {
                String::new()
            },
            emojis::RESISTANCE,
            enemy.resistance,
            emojis::VITALITY,
            enemy.vitality,
            emojis::ETHER,
            enemy.ether,
            if enemy.skills.is_empty() {
                String::from("Nenhuma")
            } else {
                enemy
                    .skills
                    .iter()
                    .map(|s| get_boxed_skill_from_kind(s.clone()).data().name.to_string())
                    .collect::<Vec<_>>()
                    .join("`, `")
            },
            if enemy_allies_fighters.is_empty() {
                String::new()
            } else {
                format!(
                    "\n⚠️ **Aliados**: `{}`",
                    enemy_allies_fighters
                        .iter()
                        .map(|a| a.name.to_owned())
                        .collect::<Vec<_>>()
                        .join("`, `")
                )
            }
        ))
        .add_field_with_emoji(
            "💰",
            EmbedField {
                name: "Recompensa".into(),
                value: format!(
                    "{} {} ◎\n🌀 {} XP{}",
                    emojis::ORB,
                    drop.orbs,
                    drop.xp,
                    if drop.items.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "\n{}",
                            drop.items
                                .iter()
                                .map(|i| format!(
                                    "**{} {}x {}**",
                                    i.item.emoji, i.amount, i.item.display_name
                                ))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                ),
                inline: false,
            },
        );

    let confirmation = ctx
        .helper()
        .create_confirmation(
            author.id,
            false,
            Response::new_user_reply(&author, "você encontrou um inimigo! Quer enfrentá-lo?")
                .add_embed(embed),
        )
        .await?;
    if !confirmation {
        return Ok(());
    }

    let mut character = parse_user_character!(ctx, author);
    if character.action_points < 2 {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você precisa de **2 pontos de ação** para explorar!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    character.action_points = character.action_points.saturating_sub(2);
    ctx.db().characters().save(character.clone()).await?;

    let mut fighters = vec![character_fighter, enemy_fighter];
    for ally in enemy_allies_fighters {
        fighters.push(ally);
    }

    let battle = Battle::new(
        character.region,
        BattleSettings {
            casual: false,
            has_consequences: true,
            is_risking_life_allowed: true,
            max_intruders: 2,
        },
        fighters,
    )?;

    let mut controller = BattleController::new(battle, &mut ctx);
    controller.run().await?;

    Ok(())
}
