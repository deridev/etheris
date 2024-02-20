use etheris_rpg::list::get_boxed_skill_from_kind;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::prelude::*;

const XP_REQUIRED_TO_LEVELUP: u32 = 100;
const KNOWLEDGE_XP_REQUIRED_TO_LEVELUP: u32 = 200;

#[command("Evolua seu personagem através do esforço mental")]
#[name("estudar")]
#[character_required(true)]
pub async fn study(mut ctx: CommandContext) -> anyhow::Result<()> {
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
        .create_cooldown(author.id, "STUDY", chrono::Duration::minutes(5))
        .await?;

    let xp = match character.stats.intelligence_level {
        0..=3 => StdRng::from_entropy().gen_range(50..=80),
        4..=10 => StdRng::from_entropy().gen_range(20..=40),
        11..=60 => StdRng::from_entropy().gen_range(10..=30),
        61..=100 => StdRng::from_entropy().gen_range(10..=15),
        _ => StdRng::from_entropy().gen_range(5..=10),
    };

    character.intelligence_xp += xp;
    character.knowledge_xp += xp;

    let levels_upgraded = character.intelligence_xp / XP_REQUIRED_TO_LEVELUP;
    let new_xp = character.intelligence_xp % XP_REQUIRED_TO_LEVELUP;

    let knowledge_levels_upgraded = character.knowledge_xp / KNOWLEDGE_XP_REQUIRED_TO_LEVELUP;
    let knowledge_new_xp = character.knowledge_xp % KNOWLEDGE_XP_REQUIRED_TO_LEVELUP;
    character.knowledge_points += knowledge_levels_upgraded;
    character.knowledge_xp = knowledge_new_xp;

    let ether_upgraded = levels_upgraded * 2;

    character.stats.ether.max += ether_upgraded as i32;
    character.stats.ether.value += ether_upgraded as i32;

    character.intelligence_xp = new_xp;
    character.stats.intelligence_level += levels_upgraded;

    let refill_minutes_upgrade = if character.stats.intelligence_level < 20 {
        levels_upgraded * 10
    } else {
        0
    };

    let mut messages: Vec<String> = Vec::with_capacity(3);
    let first_message = match character.stats.intelligence_level.saturating_sub(levels_upgraded) {
        0..=2 => format!("você leu um livro simples e recebeu **{xp} XP** em inteligência!"),
        3..=6 => format!("você leu alguns livros simples e recebeu **{xp} XP** em inteligência!"),
        7..=10 => format!("você leu um livro complicado e recebeu **{xp} XP** em inteligência!"),
        11..=15 => format!("você leu vários livros complexos e recebeu **{xp} XP** em inteligência!"),
        16..=20 => format!("você leu um livro ancestral em uma língua perdida e recebeu **{xp} XP** em inteligência!"),
        21..=30 => format!(
            "você desvendou idiomas antigos e perdidos e recebeu **{xp} XP** em inteligência!"
        ),
        _ => format!("você leu um livro milenar codificado em hieróglifos de olhos fechados enquanto meditava e recebeu **{xp} XP** em inteligência!"),
    };

    messages.push(first_message);

    if levels_upgraded > 0 {
        let levelup_message = match character.stats.intelligence_level {
            0..=2 => "Você sentiu que sua velocidade de raciocínio melhorou. Sua inteligência aumentou!",
            3..=6 => "Você começa a entender melhor a essência do ether. Sua inteligência aumentou!",
            7..=10 => "O que antes era complexo agora está simples para você. Sua inteligência aumentou!",
            11..=15 => "Sua capacidade mental agora está muito além do que você podia prever no passado. Sua inteligência aumentou!",
            21..=30 => "Problemas físicos e metafísicos envolvendo ether e o universo agora são básicos para você. Sua inteligência aumentou!",
            _ => "A sua inteligência é tão avassaladora que não há problema que você não resolva. Sua inteligência aumentou!"
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

    let aknowledged_skills = SkillKind::list()
        .iter()
        .filter(|kind| {
            !character.learnable_skills.contains(kind)
                && !character.learned_skills.contains(kind)
                && kind.intelligence_requirement() <= character.stats.intelligence_level
                && Probability::new(if kind.knowledge_cost() > 3 { 50 } else { 100 })
                    .generate_random_bool()
        })
        .cloned()
        .collect::<Vec<_>>();

    if !aknowledged_skills.is_empty() {
        messages.push(
            format!("Você descobriu as habilidades {}! Use **/aprender** para aprender elas, ou **/habilidade analisar** para investigá-las.", 
            aknowledged_skills.iter().map(|s| format!("**{}**", get_boxed_skill_from_kind(s.clone()).data().name)).collect::<Vec<_>>().join(", ")
        ));

        for skill in aknowledged_skills {
            character.aknowledge_skill(skill);
        }
    }

    if knowledge_levels_upgraded > 0 {
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
    upgrade_intelligence_level!(10); // 14 AP
    upgrade_intelligence_level!(20); // 15 AP
    upgrade_intelligence_level!(30); // 16 AP
    upgrade_intelligence_level!(40); // 17 AP
    upgrade_intelligence_level!(50); // 18 AP
    upgrade_intelligence_level!(60); // 19 AP
    upgrade_intelligence_level!(80); // 20 AP

    character.action_points -= 2;

    ctx.db().characters().save(character).await?;

    ctx.send(Response::new_user_reply(&author, messages.join("\n")))
        .await?;

    Ok(())
}
