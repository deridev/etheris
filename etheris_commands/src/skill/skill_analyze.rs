use etheris_rpg::{list::get_boxed_skill_from_kind, Fighter, FighterData};

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

    let fighter = Fighter::new(
        0,
        Default::default(),
        Default::default(),
        FighterData::new_from_character(0, &character, author.clone(), Default::default()),
    );

    let Some(skill) = character
        .learned_skills
        .iter()
        .chain(character.skills.iter())
        .map(|s| get_boxed_skill_from_kind(s.clone()))
        .find(|s| {
            unidecode::unidecode(&skill).to_lowercase()
                == unidecode::unidecode(s.data(&fighter).name).to_lowercase()
        })
    else {
        ctx.send(Response::new_user_reply(
            &author,
            format!("a habilidade **{skill}** não existe ou você ainda não aprendeu ela! Use **/habilidades** para ver as habilidades que você já sabe.")
        ).add_emoji_prefix(emojis::ERROR).set_ephemeral()).await?;

        return Ok(());
    };

    let embed = EmbedBuilder::new_common()
        .set_author_to_user(&author)
        .set_description(format!(
            "## {}\n### {}\n{}",
            skill.display(&fighter).header,
            skill.display(&fighter).sub_header,
            skill.display(&fighter).body
        ))
        .add_inlined_field("💡 Explicação:", skill.data(&fighter).explanation)
        .add_footer_text(format!(
            "Complexidade: {} | ID: {}",
            skill.data(&fighter).complexity,
            skill.data(&fighter).identifier
        ));

    ctx.send(Response::from(embed)).await?;

    Ok(())
}
