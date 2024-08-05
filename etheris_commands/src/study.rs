use std::{mem::discriminant, time::Duration};

use etheris_discord::twilight_model::channel::message::component::ButtonStyle;
use etheris_framework::{util::make_multiple_rows, watcher::WatcherOptions};
use etheris_rpg::{
    list::{get_boxed_skill_from_kind, ALL_SKILLS},
    Fighter, FighterData, SkillComplexity,
};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use crate::prelude::*;

const XP_REQUIRED_TO_LEVELUP: u32 = 150;
const KNOWLEDGE_XP_REQUIRED_TO_LEVELUP: u32 = 250;

#[command("Evolua seu personagem através do esforço mental")]
#[name("estudar")]
#[character_required(true)]
pub async fn study(mut ctx: CommandContext) -> anyhow::Result<()> {
    let guild_id = ctx.interaction.guild_id.unwrap_or(Id::new(12345678));
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);
    if character.action_points < 2 {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa de dois pontos de ação para estudar! Use **/perfil** para ver quando seus pontos de ações recarregam."
            )
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    let initial_intelligence_level = character.stats.intelligence_level;

    verify_user_cooldown!(ctx, author, "STUDY");
    ctx.db()
        .cooldowns()
        .create_cooldown(
            author.id,
            "STUDY",
            chrono::Duration::try_minutes(2).unwrap(),
        )
        .await?;

    let xp_multiplier = if guild_id == Id::new(config::GUILD_ID) {
        1.25
    } else {
        1.0
    };

    let xp = match character.stats.intelligence_level {
        0..=3 => StdRng::from_entropy().gen_range(50..=70),
        4..=100 => StdRng::from_entropy().gen_range(20..=45),
        _ => StdRng::from_entropy().gen_range(10..=30),
    };

    let xp = (xp as f64 * xp_multiplier) as u32;

    character.intelligence_xp += xp;
    character.knowledge_xp += xp + StdRng::from_entropy().gen_range(5..=10);

    let levels_upgraded = character.intelligence_xp / XP_REQUIRED_TO_LEVELUP;
    let new_xp = character.intelligence_xp % XP_REQUIRED_TO_LEVELUP;

    let knowledge_levels_upgraded = character.knowledge_xp / KNOWLEDGE_XP_REQUIRED_TO_LEVELUP;
    let knowledge_new_xp = character.knowledge_xp % KNOWLEDGE_XP_REQUIRED_TO_LEVELUP;
    character.knowledge_points += knowledge_levels_upgraded;
    character.knowledge_xp = knowledge_new_xp;

    let ether_upgraded = levels_upgraded * 1;

    character.stats.ether.max += ether_upgraded as i32;
    character.stats.ether.value += ether_upgraded as i32;

    character.intelligence_xp = new_xp;
    character.stats.intelligence_level += levels_upgraded;

    let refill_minutes_upgrade = if character.stats.intelligence_level < 30 {
        levels_upgraded * 3
    } else {
        0
    };

    let mut messages: Vec<String> = Vec::with_capacity(3);
    let first_message = match character.stats.intelligence_level.saturating_sub(levels_upgraded) {
        0..=2 => format!("você leu um livro simples e recebeu **{xp} XP** em inteligência!"),
        3..=6 => format!("você leu alguns livros simples e recebeu **{xp} XP** em inteligência!"),
        7..=10 => format!("você se dedicou à leitura de um livro mais complexo e recebeu **{xp} XP** em inteligência!"),
        11..=15 => format!("você explorou diversos temas em livros complexos e recebeu **{xp} XP** em inteligência!"),
        16..=20 => format!("você mergulhou em um livro ancestral escrito em uma língua perdida e recebeu **{xp} XP** em inteligência!"),
        21..=30 => format!("você desvendou os segredos de idiomas antigos e perdidos e recebeu **{xp} XP** em inteligência!"),
        31..=40 => format!("você absorveu conhecimento como uma esponja, expandindo seus horizontes mentais e recebendo **{xp} XP** em inteligência!"),
        41..=50 => format!("seus insights transcendem as barreiras do intelecto comum, e você ganhou **{xp} XP** em inteligência!"),
        51..=60 => format!("você se tornou um verdadeiro mestre do conhecimento, desvendando os mistérios do universo e ganhando **{xp} XP** em inteligência!"),
        61..=70 => format!("sua mente é uma fonte inesgotável de sabedoria, e você ganhou **{xp} XP** em inteligência!"),
        71..=80 => format!("você alcançou uma compreensão profunda e transcendental, e ganhou **{xp} XP** em inteligência!"),
        _ => format!("você leu um livro milenar codificado em hieróglifos de olhos fechados enquanto meditava e recebeu **{xp} XP** em inteligência!"),
    };

    messages.push(first_message);

    if levels_upgraded > 0 {
        let levelup_message = match character.stats.intelligence_level {
            0..=2 => "Você sentiu que sua velocidade de raciocínio melhorou. Sua inteligência aumentou!",
            3..=6 => "Você começa a compreender melhor os mistérios do ether. Sua inteligência aumentou!",
            7..=10 => "O que antes era complexo agora parece mais simples para você. Sua inteligência aumentou!",
            11..=15 => "Sua capacidade mental agora está muito além do que você podia prever no passado. Sua inteligência aumentou!",
            16..=20 => "Você mergulhou mais fundo nos segredos do universo, expandindo sua compreensão. Sua inteligência aumentou!",
            21..=30 => "Problemas físicos e metafísicos envolvendo o ether e o universo agora são desafios triviais para você. Sua inteligência aumentou!",
            31..=40 => "Você desvenda os enigmas do cosmos com facilidade, expandindo seus horizontes intelectuais. Sua inteligência aumentou!",
            41..=50 => "Seus insights transcendem os limites do conhecimento convencional. Sua inteligência aumentou!",
            51..=60 => "Você se tornou um verdadeiro sábio, desvendando os segredos mais profundos da existência. Sua inteligência aumentou!",
            61..=70 => "Sua mente é uma fonte de sabedoria incomparável, iluminando os mistérios do universo. Sua inteligência aumentou!",
            71..=80 => "Você alcançou uma compreensão cósmica, vendo além das fronteiras do conhecimento humano. Sua inteligência aumentou!",
            _ => "A sua inteligência é tão avassaladora que não há problema que você não seja capaz de resolver. Sua inteligência aumentou!"
        };

        messages.push(levelup_message.to_string());
    }

    if ether_upgraded > 0 {
        messages.push(format!(
            "Seu ether máximo aumentou em **{ether_upgraded}**!"
        ));
    }

    if refill_minutes_upgrade > 0 {
        character.refill_minutes -= refill_minutes_upgrade;
        messages.push(format!("Agora seus pontos de ação recarregam **{refill_minutes_upgrade} minutos** mais rápido!"));
    }

    if knowledge_levels_upgraded > 0 {
        character.insert_flag(CharacterFlag::CanAknowledgeSkill);
        messages.push(format!("Você recebeu **{knowledge_levels_upgraded} pontos de conhecimento**! Use **/aprender** para gastar aprendendo habilidades novas."));
    }

    macro_rules! upgrade_intelligence_level {
        ($x:expr) => {
            if initial_intelligence_level < $x && character.stats.intelligence_level >= $x {
                character.max_action_points += 1;
                messages.push("Seus pontos de ação máximos aumentaram em 1!".to_string());
            }
        };
    }

    upgrade_intelligence_level!(2); // 11 AP
    upgrade_intelligence_level!(3); // 12 AP
    upgrade_intelligence_level!(5); // 13 AP
    upgrade_intelligence_level!(7); // 14 AP
    upgrade_intelligence_level!(10); // 15 AP
    upgrade_intelligence_level!(15); // 16 AP
    upgrade_intelligence_level!(20); // 17 AP
    upgrade_intelligence_level!(25); // 18 AP
    upgrade_intelligence_level!(30); // 19 AP
    upgrade_intelligence_level!(35); // 20 AP
    upgrade_intelligence_level!(40); // 21 AP
    upgrade_intelligence_level!(45); // 22 AP
    upgrade_intelligence_level!(50); // 23 AP
    upgrade_intelligence_level!(55); // 24 AP
    upgrade_intelligence_level!(60); // 25 AP
    upgrade_intelligence_level!(70); // 26 AP
    upgrade_intelligence_level!(80); // 27 AP
    upgrade_intelligence_level!(90); // 28 AP
    upgrade_intelligence_level!(95); // 29 AP
    upgrade_intelligence_level!(100); // 30 AP

    character.action_points -= 2;

    ctx.send(Response::new_user_reply(&author, messages.join("\n")))
        .await?;
    ctx.db().characters().save(character.clone()).await?;

    if character.has_flag(CharacterFlag::CanAknowledgeSkill) {
        aknowledge_skill(&author, &mut ctx).await?;
    }

    Ok(())
}

pub async fn aknowledge_skill(author: &User, ctx: &mut CommandContext) -> anyhow::Result<()> {
    const MAX_SKILLS: usize = 4;
    let rng = &mut StdRng::from_entropy();
    let character = parse_user_character!(ctx, author);
    let fighter = Fighter::new(
        0,
        Default::default(),
        Default::default(),
        FighterData::new_from_character(0, &character, author.clone(), Default::default()),
    );

    let valid_skills = ALL_SKILLS
        .iter()
        .filter(|s| {
            !character
                .study_skills_cache
                .iter()
                .any(|ss| discriminant(ss) == discriminant(&s.kind()))
                && !character.already_knows_skill(s.kind())
                && s.kind().intelligence_requirement() <= character.stats.intelligence_level
        })
        .collect::<Vec<_>>();

    let Ok(new_valid_skills) = valid_skills.choose_multiple_weighted(rng, MAX_SKILLS, |s| {
        let affinities = s
            .kind()
            .personalities_affinity()
            .iter()
            .filter(|p| character.personalities.contains(*p))
            .count() as f64;

        let complexity_multiplier = match s.data(&fighter).complexity {
            SkillComplexity::VerySimple => 2.5,
            SkillComplexity::Simple => 2.0,
            SkillComplexity::Normal => 1.5,
            SkillComplexity::Hard => 1.0,
            SkillComplexity::VeryHard => 0.8,
            SkillComplexity::UltraHard => 0.6,
            SkillComplexity::BeginnerMaster => 0.4,
            SkillComplexity::Master => 0.3,
            SkillComplexity::SuperMaster => 0.1,
        };

        (3.0 + (affinities * 2.5)) * complexity_multiplier
    }) else {
        return Ok(());
    };

    // Firstly, get the skills from the cache if any.
    let mut skills = character
        .study_skills_cache
        .iter()
        .map(|s| get_boxed_skill_from_kind(s.clone()))
        .take(MAX_SKILLS)
        .collect::<Vec<_>>();

    // Then, append the new valid skills if there's space.
    if skills.len() < MAX_SKILLS {
        for skill in new_valid_skills {
            skills.push(get_boxed_skill_from_kind(skill.kind().clone()));
            if skills.len() >= MAX_SKILLS {
                break;
            }
        }
    }

    if skills.is_empty() {
        return Ok(());
    }

    let mut character = parse_user_character!(ctx, author);
    let character_name = character.name.clone();
    character.study_skills_cache = skills.iter().map(|s| s.kind()).collect();
    ctx.db().characters().save(character).await?;

    let embed = EmbedBuilder::new_common()
        .set_color(Color::VERY_LIGHT_YELLOW)
        .set_author(EmbedAuthor {
            name: format!("{} está estudando novas habilidades!", character_name),
            icon_url: Some(author.avatar_url()),
        })
        .set_title("☄️ Você descobriu novas habilidades! Escolha uma para conhecer:")
        .set_description(
            skills
                .iter()
                .map(|s| {
                    format!(
                        "## {}\n**`{} conhecimento`**\n{} **{}**\n{}",
                        s.data(&fighter).name,
                        s.kind().knowledge_cost(),
                        emojis::ETHER,
                        s.data(&fighter).use_cost.ether,
                        s.data(&fighter).description
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .add_footer_text("Descobrir uma habilidade nova é grátis! Você só gasta conhecimento ao aprender ela. (/aprender)");

    let mut buttons = vec![];
    for skill in skills.iter() {
        let button = ButtonBuilder::new()
            .set_custom_id(skill.data(&fighter).identifier)
            .set_label(skill.data(&fighter).name);

        buttons.push(button);
    }

    let message = ctx
        .send_in_channel(Response::from(embed).set_components(make_multiple_rows(buttons.clone())))
        .await?
        .model()
        .await?;

    let author_id = author.id;
    let Ok(Some(component)) = ctx
        .watcher
        .await_single_component(
            message.id,
            move |interaction| interaction.author_id() == Some(author_id),
            WatcherOptions {
                timeout: Duration::from_secs(60),
            },
        )
        .await
    else {
        return Ok(());
    };

    let data = component.parse_message_component_data()?;

    let mut ctx = CommandContext::from_with_interaction(ctx, Box::new(component));
    let buttons = buttons
        .into_iter()
        .map(|b| {
            let custom_id = b.data.custom_id.clone().unwrap_or_default();
            b.set_disabled(true)
                .set_style(if custom_id == data.custom_id {
                    ButtonStyle::Success
                } else {
                    ButtonStyle::Secondary
                })
        })
        .collect::<Vec<_>>();

    ctx.update_message(Response::default().set_components(make_multiple_rows(buttons)))
        .await?;
    let mut character = parse_user_character!(ctx, author);

    let Some(skill) = skills
        .iter()
        .find(|s| s.data(&fighter).identifier == data.custom_id)
    else {
        return Ok(());
    };

    character.study_skills_cache.clear();
    character.aknowledge_skill(skill.kind());
    character.remove_flag(CharacterFlag::CanAknowledgeSkill);
    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(author, format!("você descobriu a habilidade **{}**! Use **/aprender** para obtê-la para o seu personagem.", skill.data(&fighter).name))
            .add_emoji_prefix("💡")
    ).await?;

    Ok(())
}
