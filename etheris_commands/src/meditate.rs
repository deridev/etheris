use std::time::Duration;

use etheris_rpg::{Battle, BattleController, BattleSettings, FighterData};
use personality::Personality;

use crate::prelude::*;

#[rustfmt::skip]
#[command("Evolua seu personagem através do esforço espiritual")]
#[name("meditar")]
#[character_required(true)]
pub async fn meditate(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    verify_user_cooldown!(ctx, author, "MEDITATE");
    
    let character = parse_user_character!(ctx, author);
    let confirmation_embed = EmbedBuilder::new_common()
        .set_color(Color::LIGHT_YELLOW)
        .set_author_to_user(&author)
        .set_description(format!("## Quer iniciar sua meditação? 🧘\nO seu nível espiritual é **{}**. Você terá que enfrentar a si mesmo e superar os seus limites para evoluir!\nMate seu espírito para renascer mais forte.", character.mental_level))
        .add_footer_text("Meditar custará 3 pontos de ação");        
    let confirmation = ctx.helper().create_confirmation(author.id, false, confirmation_embed).await?;
    if !confirmation {
        return Ok(());
    }

    let character = parse_user_character!(ctx, author);
    if character.action_points < 3 {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa de **3 pontos de ação** para meditar! Use **/perfil** para ver quando seus pontos de ações recarregam."
            )
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    verify_user_cooldown!(ctx, author, "MEDITATE");
    ctx.db()
        .cooldowns()
        .create_cooldown(author.id, "MEDITATE", chrono::Duration::try_minutes(10).unwrap())
        .await?;

    let inner_shadow = create_inner_shadow(author.clone(), &character); 
    let inner_shadow_name = inner_shadow.name.clone();

    let fighter = FighterData::new_from_character(0, &character, author.clone(), Default::default());

    let battle = Battle::new(
        character.region,
        BattleSettings {
            is_risking_life_allowed: true,
            has_consequences: false,
            casual: true,
            max_intruders: 0,
        },
        vec![fighter, inner_shadow],
    )?;

    let mut controller = BattleController::new(battle, ctx);
    let result = controller.run().await?;
    let mut ctx = controller.ctx.clone();

    let all_fighters = result.losers.iter().chain(result.winners.iter()).cloned().collect::<Vec<_>>();

    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut character = parse_user_character!(ctx, author);
    character.action_points = character.action_points.saturating_sub(3);

    let (_, inner_shadow_fighter) = {
        let author_fighter = all_fighters.iter().find(|f| f.team == 0 && f.user.is_some());
        let inner_shadow_fighter = all_fighters.iter().find(|f| f.team == 1 && f.name == inner_shadow_name);

        (author_fighter.context("expected a valid author fighter")?, inner_shadow_fighter.context("expected a valid inner shadow fighter")?)
    };

    let has_won = result.winners.iter().any(|w| w.team == 0);

    let total_dmg_dealt_ratio = 1.0 - inner_shadow_fighter.health().value as f32 / (inner_shadow_fighter.health().max as f32);

    if has_won {
        let mut knowledge_xp = 25 + (total_dmg_dealt_ratio * 60.0).round() as u32;
        let mut intelligence_xp = 70 + (total_dmg_dealt_ratio * 300.0).round() as u32;

        knowledge_xp = (knowledge_xp as f64 * character.mental_level.reward_multiplier()) as u32;
        intelligence_xp = (intelligence_xp as f64 * character.mental_level.reward_multiplier()) as u32;

        let next_mental_level = match character.mental_level {
            MentalLevel::Laymen => Some(MentalLevel::Beginner),
            MentalLevel::Beginner => Some(MentalLevel::Novice),
            MentalLevel::Novice => Some(MentalLevel::Accustomed),
            MentalLevel::Accustomed => Some(MentalLevel::Spirited),
            MentalLevel::Spirited => Some(MentalLevel::Experient),
            MentalLevel::Experient => Some(MentalLevel::Strong),
            MentalLevel::Strong => Some(MentalLevel::Master),
            MentalLevel::Master => Some(MentalLevel::Champion),
            MentalLevel::Champion => Some(MentalLevel::Legend),
            MentalLevel::Legend => None,
        };

        if next_mental_level.is_none() {
            knowledge_xp = (knowledge_xp as f32 * 0.3).round() as u32 + 2;
            intelligence_xp = (intelligence_xp as f32 * 0.2).round() as u32 + 2;
        }
        
        let mut extras = vec![];

        character.knowledge_xp += knowledge_xp;
        character.intelligence_xp += intelligence_xp;
        if let Some(next_level) = next_mental_level {
            extras.push("**Você liberou 10% do seu potencial oculto!**".to_string());
            character.potential += 0.1;
            character.mental_level = next_level;
        }

        if MentalLevel::Accustomed == character.mental_level {
            character.actions.insert(BattleAction::ControlPower);
            extras.push("**Agora você é capaz de controlar o seu poder no meio de batalhas!**".to_string());
        }

        const MENTAL_PACT_MAX: &[(MentalLevel, u8)] = &[
            (MentalLevel::Spirited, 2),
            (MentalLevel::Champion, 3),
        ];

        for (level, pact_count) in MENTAL_PACT_MAX {
            if level.level() <= character.mental_level.level() && character.max_pacts < *pact_count {
                extras.push(format!("**Agora você pode formar {pact_count} pactos!**"));
                character.max_pacts = *pact_count;
            }
        }

        ctx.db().characters().save(character).await?;

        ctx.send(
            Response::new_user_reply(
                &author,
                format!(
                    "**você venceu o seu espírito interior**!\nVocê ganhou **{} XP** de conhecimento e **{} XP** de inteligência {}{}",
                    knowledge_xp,
                    intelligence_xp,
                    if let Some(next_mental_level) = next_mental_level {
                        format!("e alcançou um novo estágio espiritual, o estágio **{}**!", next_mental_level)
                    } else {
                        "mas já está no estágio máximo de espírito!".to_string()
                    },
                    if extras.is_empty() {
                        String::new()
                    } else {
                        format!("\n{}", extras.join("\n"))
                    }
                )
            ).add_emoji_prefix("🧠🧘")
        ).await?;
    } else {
        let knowledge_xp = 1 + (total_dmg_dealt_ratio * 10.0).round() as u32;
        let intelligence_xp = 3 + (total_dmg_dealt_ratio * 30.0).round() as u32;

        ctx.db().characters().save(character).await?;

        ctx.send(
            Response::new_user_reply(
                &author,
                format!(
                    "**você meditou mas não conseguiu se superar**!\nVocê ganhou **{} XP** de conhecimento e **{} XP** de inteligência. Tente novamente mais tarde!",
                    knowledge_xp,
                    intelligence_xp,
                )
            ).add_emoji_prefix("🧘☹️")
        ).await?;
    }

    Ok(())
}

fn create_inner_shadow(dummy: User, character: &CharacterModel) -> FighterData {
    let overall_multiplier = match character.mental_level {
        MentalLevel::Laymen => 0.5,
        MentalLevel::Beginner => 0.75,
        MentalLevel::Novice => 1.0,
        MentalLevel::Accustomed => 1.5,
        MentalLevel::Spirited => 1.8,
        MentalLevel::Experient => 2.25,
        MentalLevel::Strong => 3.0,
        MentalLevel::Master => 6.0,
        MentalLevel::Legend => 8.0,
        MentalLevel::Champion => 13.0,
    };

    let dmg_multiplier = match character.mental_level {
        MentalLevel::Laymen => 0.5,
        MentalLevel::Beginner => 0.6,
        MentalLevel::Novice => 0.8,
        MentalLevel::Accustomed => 1.0,
        MentalLevel::Spirited => 1.2,
        MentalLevel::Experient => 1.4,
        MentalLevel::Strong => 1.6,
        MentalLevel::Master => 2.0,
        MentalLevel::Legend => 2.3,
        MentalLevel::Champion => 2.6,
    };

    let mut shadow_character = character.clone();
    shadow_character.potential += 0.2;
    shadow_character.personalities.push(Personality::Insanity);

    macro_rules! multiply_by_multiplier {
        ($value:expr, $multiplier:expr) => {
            $value = ($value as f64 * $multiplier) as u32;
        };
        ($value:expr, $multiplier:expr, $type:ty) => {
            $value = ($value as f64 * $multiplier) as $type;
        };
    }

    multiply_by_multiplier!(shadow_character.stats.strength_level, dmg_multiplier * 0.8);
    multiply_by_multiplier!(
        shadow_character.stats.intelligence_level,
        dmg_multiplier * 0.8
    );
    multiply_by_multiplier!(shadow_character.stats.health_level, overall_multiplier);

    multiply_by_multiplier!(
        shadow_character.stats.ether.value,
        0.3 + overall_multiplier * 0.4,
        i32
    );
    multiply_by_multiplier!(
        shadow_character.stats.ether.max,
        0.3 + overall_multiplier * 0.4,
        i32
    );

    multiply_by_multiplier!(
        shadow_character.stats.resistance.value,
        overall_multiplier,
        i32
    );
    multiply_by_multiplier!(
        shadow_character.stats.resistance.max,
        overall_multiplier,
        i32
    );

    multiply_by_multiplier!(
        shadow_character.stats.vitality.value,
        overall_multiplier,
        i32
    );
    multiply_by_multiplier!(shadow_character.stats.vitality.max, overall_multiplier, i32);

    let mut fighter =
        FighterData::new_from_character(1, &shadow_character, dummy, Default::default());
    fighter.user = None;
    fighter.name = "Sombra Interior".to_string();
    fighter.brain = Some(BrainKind::Boss);

    fighter
}
