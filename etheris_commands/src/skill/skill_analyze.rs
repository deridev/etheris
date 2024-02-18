use etheris_rpg::list::get_boxed_skill_from_kind;

use crate::prelude::*;

#[command("Analise uma habilidade que você já aprendeu!")]
#[name("habilidade analisar")]
#[character_required(true)]
pub async fn skill_analyze(
    mut ctx: CommandContext,
    #[rename("habilidade")]
    #[description("O nome da habilidade que você quer analisar")]
    #[min_max_length(1, 128)]
    skill: String,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character: CharacterModel = parse_user_character!(ctx, author);

    let Some(skill) = character
        .learned_skills
        .iter()
        .map(|s| get_boxed_skill_from_kind(s.clone()))
        .find(|s| {
            unidecode::unidecode(&skill).to_lowercase()
                == unidecode::unidecode(s.data().name).to_lowercase()
        })
    else {
        ctx.send(Response::new_user_reply(
            &author,
            format!("a habilidade **{skill}** não existe ou você ainda não aprendeu ela! Use **/habilidades** para ver as habilidades que você já sabe.")
        ).add_emoji_prefix(emojis::ERROR).set_ephemeral()).await?;

        return Ok(());
    };

    let confirmation = ctx.helper()
        .create_confirmation(author.id, false,
                Response::new_user_reply(
                    &author,
                    format!("analisar uma habilidade custa tempo e custará **1 ponto de ação!**\nQuer mesmo analisar a habilidade **{}**?", skill.data().name)
                ).set_ephemeral().add_emoji_prefix("❓")
            ).await?;

    if !confirmation {
        return Ok(());
    }

    let mut character: CharacterModel = parse_user_character!(ctx, author);
    if character.action_points < 1 {
        ctx.send(Response::new_user_reply(&author, "você não tem 1 ponto de ação! Use **/perfil** para ver quando seus pontos de ação recarregam.").add_emoji_prefix(emojis::ERROR).set_ephemeral()).await?;
        return Ok(());
    }

    character.action_points -= 1;
    ctx.db().characters().save(character).await?;

    let embed = EmbedBuilder::new_common()
        .set_author_to_user(&author)
        .set_description(format!(
            "## {}\n### {}\n{}",
            skill.display().header,
            skill.display().sub_header,
            skill.display().body
        ))
        .add_inlined_field("💡 Explicação:", skill.data().explanation)
        .add_footer_text(format!(
            "Complexidade: {} | ID: {}",
            skill.data().complexity,
            skill.data().identifier
        ));

    ctx.followup_interaction(Response::from(embed).set_ephemeral())
        .await?;

    Ok(())
}
