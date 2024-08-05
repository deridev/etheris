use etheris_rpg::{list::get_boxed_skill_from_kind, Fighter, FighterData};

use crate::prelude::*;

#[command("Equipe uma habilidade que você já aprendeu!")]
#[name("habilidade equipar")]
#[character_required(true)]
pub async fn skill_equip(
    mut ctx: CommandContext,
    #[rename("habilidade")]
    #[description("O nome da habilidade que você quer equipar")]
    #[min_max_length(1, 128)]
    skill: String,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character: CharacterModel = parse_user_character!(ctx, author);
    let fighter = Fighter::new(
        0,
        Default::default(),
        Default::default(),
        FighterData::new_from_character(0, &character, author.clone(), Default::default()),
    );

    if character.skills.len() >= 8 {
        ctx.send(Response::new_user_reply(
            &author,
            "você já equipou o máximo de habilidades permitidas por personagem! Use **/habilidade desequipar** para remover uma habilidade equipada."
        ).add_emoji_prefix(emojis::ERROR)).await?;

        return Ok(());
    }

    let Some(skill) = character
        .learned_skills
        .iter()
        .map(|s| get_boxed_skill_from_kind(s.clone()))
        .find(|s| {
            unidecode::unidecode(&skill).to_lowercase()
                == unidecode::unidecode(s.data(&fighter).name).to_lowercase()
        })
    else {
        ctx.send(Response::new_user_reply(
            &author,
            format!("a habilidade **{skill}** já está equipada, não existe ou você ainda não aprendeu ela! Use **/habilidades** para ver as habilidades que você já sabe.")
        ).add_emoji_prefix(emojis::ERROR)).await?;

        return Ok(());
    };

    if character.skills.contains(&skill.kind()) {
        return Ok(());
    }

    character.equip_skill(skill.kind());
    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "a habilidade **{}** foi equipada com sucesso!",
                skill.data(&fighter).name
            ),
        )
        .add_emoji_prefix(emojis::SUCCESS),
    )
    .await?;

    Ok(())
}
