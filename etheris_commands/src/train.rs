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
    verify_user_cooldown!(ctx, author, "TRAIN");
    
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
        _ => StdRng::from_entropy().gen_range(10..=30),
    }
}

const XP_REQUIRED_TO_LEVELUP: u32 = 100;

pub async fn strength_training(
    mut ctx: CommandContext,
    mut character: CharacterModel,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let xp = random_xp_amount(character.stats.strength_level);
    character.strength_xp += xp;

    let levels_upgraded = character.strength_xp / XP_REQUIRED_TO_LEVELUP;
    let new_xp = character.strength_xp % XP_REQUIRED_TO_LEVELUP;

    character.strength_xp = new_xp;
    character.stats.strength_level += levels_upgraded;

    let mut messages: Vec<String> = Vec::with_capacity(3);
    let first_message = match character.stats.strength_level - levels_upgraded {
        0..=2 => format!("você fez alguns exercícios leves e recebeu **{xp} XP** em força!"),
        3..=6 => format!("você fez vários exercícios leves e recebeu **{xp} XP** em força!"),
        7..=10 => format!("você se dedicou a treinos moderados e ganhou **{xp} XP** em força!"),
        11..=15 => format!("você intensificou seus treinos e ganhou **{xp} XP** em força!"),
        16..=20 => format!("você se dedicou a levantamento de peso e ganhou **{xp} XP** em força!"),
        21..=30 => format!("você se tornou uma verdadeira força da natureza e ganhou **{xp} XP** em força!"),
        31..=40 => format!("seu poder é lendário entre os guerreiros e ganhou **{xp} XP** em força!"),
        41..=50 => format!("sua força é comparável a de um titã e ganhou **{xp} XP** em força!"),
        51..=60 => format!("você é uma verdadeira força inabalável e ganhou **{xp} XP** em força!"),
        61..=70 => format!("você transcendeu os limites da força mortal e ganhou **{xp} XP** em força!"),
        71..=80 => format!("você alcançou um nível divino de força e ganhou **{xp} XP** em força!"),
        _ => format!("você se tornou uma lenda viva, uma verdadeira força da natureza, e ganhou **{xp} XP** em força!"),
    };

    messages.push(first_message);

    if levels_upgraded > 0 {
        let levelup_message = match character.stats.strength_level {
            0..=2 => "Você sentiu que seus músculos estão mais rígidos e fortes. Sua força aumentou!",
            3..=6 => "Você notou que seu corpo está maior e mais resistente. Sua força aumentou!",
            7..=10 => "O que antes era pesado agora está mais leve para você. Sua força aumentou!",
            11..=15 => "Sua capacidade física agora está muito além do que você podia prever no passado. Sua força aumentou!",
            16..=20 => "Seus músculos se tornaram densos como pedra. Sua força aumentou!",
            21..=30 => "Você se tornou uma verdadeira força da natureza. Sua força aumentou!",
            31..=40 => "Seus músculos são como montanhas intransponíveis. Sua força aumentou!",
            41..=50 => "Sua força é comparável à dos lendários heróis da mitologia. Sua força aumentou!",
            51..=60 => "Você alcançou um nível de força que poucos podem igualar. Sua força aumentou!",
            61..=70 => "Você transcendeu os limites da força mortal. Sua força aumentou!",
            71..=80 => "Seu poder rivaliza com os maiores guerreiros da história. Sua força aumentou!",
            _ => "A sua força é tão grande que sua mera presença inspira respeito e temor. Sua força aumentou!"
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
    character.stats.resistance.max += (levels_upgraded * 9) as i32;
    character.stats.resistance.value += (levels_upgraded * 9) as i32;

    character.stats.vitality.max += (levels_upgraded * 12) as i32;
    character.stats.vitality.value += (levels_upgraded * 12) as i32;

    let mut messages: Vec<String> = Vec::with_capacity(3);
    let first_message = match character.stats.health_level - levels_upgraded {
        0..=2 => format!("você fez alguns exercícios leves e recebeu **{xp} XP** em vida!"),
        3..=5 => format!("você fez vários exercícios leves e recebeu **{xp} XP** em vida!"),
        6..=8 => format!("você praticou yoga e meditação, ganhando **{xp} XP** em saúde!"),
        9..=11 => format!("você correu maratonas, melhorando sua resistência e ganhando **{xp} XP** em vida!"),
        12..=15 => format!("você fez uma rotina intensa de cardio e ganhou **{xp} XP** em saúde!"),
        16..=18 => format!("você levantou pesos pesados, fortalecendo seus músculos e ganhando **{xp} XP** em vida!"),
        19..=22 => format!("você treinou intensamente, ganhando **{xp} XP** em saúde!"),
        23..=26 => format!("você fez um treino de resistência incrível, aumentando sua força e ganhando **{xp} XP** em vida!"),
        27..=30 => format!("você superou seus limites e fez um treino extremamente intenso, ganhando **{xp} XP** em saúde!"),
        31..=35 => format!("você alcançou um novo patamar de condicionamento físico, ganhando **{xp} XP** em vida!"),
        36..=40 => format!("seu treinamento excepcional elevou sua saúde para novas alturas, concedendo **{xp} XP**!"),
        41..=45 => format!("você se tornou uma máquina de exercícios, ganhando **{xp} XP** em vida!"),
        46..=50 => format!("sua dedicação aos treinos é incomparável, ganhando **{xp} XP** em saúde!"),
        51..=60 => format!("você é um verdadeiro atleta, ganhando **{xp} XP** em vida por cada treino!"),
        61..=70 => format!("seu comprometimento com a saúde é inspirador, ganhando **{xp} XP** em saúde a cada esforço!"),
        71..=80 => format!("você transcendeu os limites do corpo humano, ganhando **{xp} XP** em vida por cada movimento!"),
        81..=90 => format!("você é uma força da natureza, ganhando **{xp} XP** em saúde por cada respiração!"),
        91..=100 => format!("seu domínio sobre o corpo é lendário, ganhando **{xp} XP** em vida por cada desafio superado!"),
        _ => format!("você alcançou um estado de perfeição física, ganhando **{xp} XP** em saúde por cada ação!")
    };

    messages.push(first_message);

    if levels_upgraded > 0 {
        let levelup_message = match character.stats.health_level {
            0..=2 => {
                "Você sentiu que seu corpo não está tão frágil quanto antes. Sua vida aumentou!"
            }
            3..=6 => "Você notou que seu corpo agora é mais resistente. Sua vida aumentou!",
            7..=10 => {
                "O que antes te feria muito agora causa apenas ferimentos leves. Sua vida aumentou!"
            }
            11..=15 => {
                "Sua resistência ultrapassou suas expectativas anteriores. Sua vida aumentou!"
            }
            16..=20 => "Você se tornou mais robusto e resistente a danos. Sua vida aumentou!",
            21..=30 => "Agora precisa de muita força para te parar. Sua vida aumentou!",
            31..=40 => "Você se tornou uma muralha de resistência. Sua vida aumentou!",
            41..=50 => "Seu corpo se transformou em uma fortaleza inabalável. Sua vida aumentou!",
            51..=60 => "Você é uma verdadeira fortaleza ambulante. Sua vida aumentou!",
            61..=70 => "Sua resistência é lendária entre os guerreiros. Sua vida aumentou!",
            71..=80 => "Os deuses invejam sua durabilidade. Sua vida aumentou!",
            81..=90 => "Você transcendeu os limites da mortalidade. Sua vida aumentou!",
            91..=100 => {
                "Você atingiu um nível quase inatingível de resistência. Sua vida aumentou!"
            }
            _ => "Sua resistência é tão grande que desafia até os deuses. Sua vida aumentou!",
        };

        messages.push(levelup_message.to_string());
    }

    ctx.db().characters().save(character).await?;

    ctx.send(Response::new_user_reply(&author, messages.join("\n")))
        .await?;

    Ok(())
}
