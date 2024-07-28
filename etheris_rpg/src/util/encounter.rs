use etheris_common::Color;
use etheris_data::{emojis, items::get_item_by_weapon};
use etheris_discord::{bold, twilight_model::user::User, EmbedBuilder};
use etheris_framework::*;

use crate::{
    events::list::prelude::Reward, list::get_boxed_skill_from_kind, Battle, BattleController,
    BattleResult, BattleSettings, Fighter, FighterData,
};

pub async fn prompt_encounter(
    ctx: &mut CommandContext,
    user: User,
    enemies: Vec<FighterData>,
    allies: Vec<FighterData>,
) -> anyhow::Result<Option<BattleResult>> {
    let Some(character) = ctx
        .db()
        .characters()
        .get_by_user(&user.id.to_string())
        .await?
    else {
        return Ok(Default::default());
    };

    let reward = enemies
        .iter()
        .fold(Reward::default(), |acc, x| acc.add(x.drop.clone()));

    let enemies_pl = enemies.iter().fold(0, |acc, x| acc + x.power_level());
    let power_diff = (character.pl - enemies_pl) / 2;

    let color = match power_diff {
        i64::MIN..=-100 => Color::RED,
        -98..=-6 => Color::LIGHT_RED,
        -5..=10 => Color::YELLOW,
        11..=30 => Color::LIGHT_BLUE,
        31..=100 => Color::LIGHT_GREEN,
        _ => Color::GREEN,
    };

    let mut embed = EmbedBuilder::new_common()
        .set_author_to_user(&user)
        .set_color(color);

    if let Some(boss) = enemies.iter().find_map(|e| e.boss) {
        embed = embed.add_description_text(format!(
            "# {}\n*{}*\n",
            boss.name(),
            boss.short_description()
        ));
    }

    let warning = if character.stats.resistance.value < character.stats.resistance.max {
        "## ⚠️ **__Você não está com a vida cheia!__**\nTome cuidado ou descanse antes de batalhar.\n"
    } else {
        ""
    };

    if !warning.is_empty() {
        embed = embed.add_description_text(warning);
    }

    for enemy in enemies.iter() {
        let strength_message =
            match (character.stats.strength_level as i64 - enemy.strength_level as i64) / 2 {
                i64::MIN..=-100 => "Extremamente mais forte",
                -60..=-30 => "Muito mais forte",
                -29..=-11 => "Mais forte",
                -10..=-3 => "Pouco mais forte",
                -2..=2 => "Semelhante",
                3..=14 => "Pouco mais fraco",
                15..=30 => "Mais fraco",
                31..=60 => "Muito mais fraco",
                61..=1000000 => "Extremamente mais fraco",
                _ => "Incalculável",
            };

        let intelligence_message =
            match (character.stats.intelligence_level as i64 - enemy.intelligence_level as i64) / 2
            {
                i64::MIN..=-100 => "Extremamente menos inteligente",
                -60..=-30 => "Muito mais inteligente",
                -29..=-11 => "Mais inteligente",
                -10..=-3 => "Pouco mais inteligente",
                -2..=2 => "Semelhante",
                3..=14 => "Pouco menos inteligente",
                15..=30 => "Menos inteligente",
                31..=60 => "Muito menos inteligente",
                61..=1000000 => "Extremamente menos inteligente",
                _ => "Incalculável",
            };

        let weapon_text = match enemy.weapon {
            Some(weapon) => {
                let item = get_item_by_weapon(weapon);
                format!("**Arma**: {} {}\n", item.display_name, item.emoji)
            }
            None => Default::default(),
        };

        let dummy_enemy_fighter = Fighter::dummy(enemy.clone());

        let skills_string = if enemy.boss.is_some() {
            String::new()
        } else {
            format!(
                "\n\n**Habilidades**: `{}`",
                if enemy.skills.is_empty() {
                    String::from("Nenhuma")
                } else {
                    enemy
                        .skills
                        .iter()
                        .map(|s| {
                            get_boxed_skill_from_kind(s.clone())
                                .data(&dummy_enemy_fighter)
                                .name
                                .to_string()
                        })
                        .collect::<Vec<_>>()
                        .join("`, `")
                }
            )
        };

        embed = embed.add_inlined_field(
            &enemy.name,
            format!(
                "{}\n{weapon_text}{} **{}**\n{} {}\n{} {}\n**Força**: {strength_message}\n**Inteligência**: {intelligence_message}{skills_string}",
                enemy.personalities.iter().map(|p| bold(&p.to_string())).collect::<Vec<_>>().join(", "),
                emojis::RESISTANCE, (enemy.resistance.value as f64) as i64,
                emojis::VITALITY, (enemy.vitality.value as f64) as i64,
                emojis::ETHER, (enemy.ether.value as f64) as i64,
            ),
        );
    }

    if !reward.is_empty() {
        embed = embed.add_inlined_field(
            "💰 Recompensas:",
            format!(
                "{} {} ◎\n{} {} XP\n{}",
                emojis::ORB,
                reward.orbs,
                emojis::XP,
                reward.xp,
                reward
                    .items
                    .iter()
                    .map(|i| format!("{} **{}x {}**", i.item.emoji, i.amount, i.item.display_name))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
    }

    let confirmation = ctx
        .helper()
        .create_confirmation(
            user.id,
            false,
            Response::new_user_reply(&user, "você quer entrar em uma batalha?")
                .add_embed(embed)
                .add_emoji_prefix("⚔️"),
        )
        .await?;

    if !confirmation {
        return Ok(None);
    }

    let mut fighters = enemies;
    fighters.insert(
        0,
        FighterData::new_from_character(0, &character, user, Default::default()),
    );
    fighters.extend_from_slice(&allies);

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

    let mut controller = BattleController::new(battle, ctx.clone());
    Ok(Some(controller.run().await?))
}
