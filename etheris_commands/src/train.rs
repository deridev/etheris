use std::time::Duration;

use etheris_discord::twilight_model::channel::message::component::ButtonStyle;
use etheris_framework::watcher::WatcherOptions;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::prelude::*;

#[rustfmt::skip]
#[command("Evolua seu personagem através do esforço físico")]
#[name("treinar")]
#[character_required(true)]
pub async fn train(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    if character.action_points == 0 {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você não tem nenhum ponto de ação para treinar! Use **/perfil** para ver quando seus pontos de ações recarregam."
            )
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    verify_user_cooldown!(ctx, author, "TRAIN");
    ctx.db()
        .cooldowns()
        .create_cooldown(author.id, "TRAIN", chrono::Duration::minutes(3))
        .await?;

    let buttons = vec![
        ButtonBuilder::new().set_custom_id("strength").set_label("Força").set_emoji(Emoji::from_unicode("💪")),
        ButtonBuilder::new().set_custom_id("health").set_label("Vida").set_emoji(emojis::VITALITY),
    ];
    let row = ActionRowBuilder::new().add_buttons(buttons.clone());

    let message = ctx.send(Response::new_user_reply(&author, "escolha o que você quer treinar ao custo de **1 ponto de ação**:").set_components(vec![row]).add_emoji_prefix("🏋️")).await?;

    let Ok(Some(collected)) = ctx.watcher.await_single_component(message.id, move |interaction| interaction.author_id() == Some(author.id), WatcherOptions {
        timeout: Duration::from_secs(60)
    }).await else {
        return Ok(());
    };

    let data = collected.parse_message_component_data()?;

    let buttons = buttons.into_iter().map(|b| {
        let custom_id = b.data.custom_id.clone().unwrap_or_default();
        b.set_disabled(true).set_style(if custom_id == data.custom_id { ButtonStyle::Success } else { ButtonStyle::Secondary })
    }).collect::<Vec<_>>();

    let row = ActionRowBuilder::new().add_buttons(buttons);

    let mut interaction_ctx = CommandContext::from_with_interaction(&ctx, collected.into());
    interaction_ctx.update_message(Response::default().set_components(vec![row])).await?;

    // The actual training
    let mut character = parse_user_character!(ctx, author);
    if character.action_points == 0 {
        return Ok(());
    }

    character.action_points -= 1;

    match data.custom_id.as_str() {
        "strength" => {
            strength_training(ctx, character).await?;
        },
        "health" => {
            health_training(ctx, character).await?;
        }
        _ => return Ok(())
    }

    Ok(())
}

fn random_xp_amount(level: u32) -> u32 {
    match level {
        0..=3 => StdRng::from_entropy().gen_range(50..=70),
        4..=10 => StdRng::from_entropy().gen_range(20..=45),
        11..=30 => StdRng::from_entropy().gen_range(10..=30),
        41..=100 => StdRng::from_entropy().gen_range(10..=15),
        _ => StdRng::from_entropy().gen_range(5..=10),
    }
}

const XP_REQUIRED_TO_LEVELUP: u32 = 100;

pub async fn strength_training(
    mut ctx: CommandContext,
    mut character: CharacterModel,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let xp = (random_xp_amount(character.stats.strength_level) as f32 * 0.8) as u32;
    character.strength_xp += xp;

    let levels_upgraded = character.strength_xp / XP_REQUIRED_TO_LEVELUP;
    let new_xp = character.strength_xp % XP_REQUIRED_TO_LEVELUP;

    character.strength_xp = new_xp;
    character.stats.strength_level += levels_upgraded;

    let mut messages: Vec<String> = Vec::with_capacity(3);
    let first_message = match character.stats.strength_level - levels_upgraded {
        0..=2 => format!("você fez alguns exercícios leves e recebeu **{xp} XP** em força!"),
        3..=6 => format!("você fez vários exercícios leves e recebeu **{xp} XP** em força!"),
        7..=10 => format!("você fez exercícios pesados e recebeu **{xp} XP** em força!"),
        11..=15 => format!("você fez levantamento de peso e recebeu **{xp} XP** em força!"),
        16..=20 => format!("você fez levantamento de peso pesado e recebeu **{xp} XP** em força!"),
        21..=30 => format!(
            "você fez levantamento de peso EXTREMAMENTE pesado e recebeu **{xp} XP** em força!"
        ),
        _ => format!("você treinou levantando árvores enormes e recebeu **{xp} XP** em força!"),
    };

    messages.push(first_message);

    if levels_upgraded > 0 {
        let levelup_message = match character.stats.strength_level {
            0..=2 => "Você sentiu que seus músculos estão mais rígidos e fortes. Sua força aumentou!",
            3..=6 => "Você notou que seu corpo está maior e mais resistente. Sua força aumentou!",
            7..=10 => "O que antes era pesado agora está leve para você. Sua força aumentou!",
            11..=15 => "Sua capacidade física agora está muito além do que você podia prever no passado. Sua força aumentou!",
            21..=30 => "Seus músculos estão mais duros que aço. Sua força aumentou!",
            _ => "A sua força é tamanha que sua mera presença afeta a pressão do ar em volta. Sua força aumentou!"
        };

        messages.push(levelup_message.to_string());
    }

    ctx.db().characters().save(character).await?;

    ctx.send(Response::new_user_reply(&author, messages.join("\n")))
        .await?;

    Ok(())
}

pub async fn health_training(
    mut ctx: CommandContext,
    mut character: CharacterModel,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let xp = random_xp_amount(character.stats.health_level);
    character.health_xp += xp;

    let levels_upgraded = character.health_xp / XP_REQUIRED_TO_LEVELUP;
    let new_xp = character.health_xp % XP_REQUIRED_TO_LEVELUP;

    character.health_xp = new_xp;
    character.stats.health_level += levels_upgraded;
    character.stats.resistance.max += (levels_upgraded * 6) as i32;
    character.stats.resistance.value += (levels_upgraded * 6) as i32;

    character.stats.vitality.max += (levels_upgraded * 8) as i32;
    character.stats.vitality.value += (levels_upgraded * 8) as i32;

    let mut messages: Vec<String> = Vec::with_capacity(3);
    let first_message = match character.stats.health_level - levels_upgraded {
        0..=2 => format!("você fez alguns exercícios leves e recebeu **{xp} XP** em vida!"),
        3..=5 => format!("você fez vários exercícios leves e recebeu **{xp} XP** em vida!"),
        7..=10 => format!("você fez exercícios pesados e recebeu **{xp} XP** em vida!"),
        11..=15 => format!("você fez levantamento de peso e recebeu **{xp} XP** em vida!"),
        16..=20 => format!("você fez levantamento de peso pesado e recebeu **{xp} XP** em vida!"),
        21..=30 => format!(
            "você fez levantamento de peso EXTREMAMENTE pesado e recebeu **{xp} XP** em vida!"
        ),
        _ => format!("você treinou levantando árvores enormes e recebeu **{xp} XP** em vida!"),
    };

    messages.push(first_message);

    if levels_upgraded > 0 {
        let levelup_message = match character.stats.health_level {
            0..=2 => "Você sentiu que seu corpo não está tão frágil quanto antes. Sua vida aumentou!",
            3..=6 => "Você notou que seu corpo agora é mais resistente. Sua vida aumentou!",
            7..=10 => "O que antes que feria muito agora te faz ferimentos leves. Sua vida aumentou!",
            11..=15 => "Sua resistência ultrapassou suas expectativas de antigamente. Sua vida aumentou!",
            21..=30 => "Agora precisa de muita força pra poder te parar. Sua vida aumentou!",
            _ => "A sua resistência é tanta que te derrotar é um feito para poucos no mundo. Sua vida aumentou!"
        };

        messages.push(levelup_message.to_string());
    }

    ctx.db().characters().save(character).await?;

    ctx.send(Response::new_user_reply(&author, messages.join("\n")))
        .await?;

    Ok(())
}
