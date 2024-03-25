use etheris_rpg::{list::get_boxed_skill_from_kind, Fighter, FighterData};

use crate::prelude::*;

#[command("Veja as habilidades de um personagem em Etheris")]
#[name("habilidades")]
#[character_required(true)]
pub async fn skills(
    mut ctx: CommandContext,
    #[rename("usuário")]
    #[description("O usuário que você quer ver as habilidades")]
    user: Option<User>,
) -> anyhow::Result<()> {
    let user = user.unwrap_or(ctx.author().await?);
    let character = parse_user_character!(ctx, user);
    let fighter = Fighter::new(
        0,
        Default::default(),
        Default::default(),
        FighterData::new_from_character(0, &character, user.clone(), Default::default()),
    );

    let image = character.create_image_bufer();
    let attachment =
        image.map(|image| DiscordAttachment::from_bytes("image.png".to_owned(), image, 1));

    let skills = character
        .skills
        .iter()
        .map(|kind| get_boxed_skill_from_kind(kind.clone()))
        .collect::<Vec<_>>();
    let learned_skills = character
        .learned_skills
        .iter()
        .map(|kind| get_boxed_skill_from_kind(kind.clone()))
        .collect::<Vec<_>>();

    let embed = EmbedBuilder::new()
        .set_author(EmbedAuthor {
            name: format!("Habilidades do personagem {}", character.name),
            icon_url: Some(user.avatar_url()),
        })
        .set_color(Color::BLURPLE)
        .set_description(format!(
            "**{} pontos de conhecimento**\n`({} XP em conhecimento)`\n{}",
            character.knowledge_points,
            character.knowledge_xp,
            if character.knowledge_points > 0 {
                "\nUse **/aprender** para gastar ponto e aprender habilidades novas."
            } else {
                ""
            }
        ))
        .set_thumbnail(if let Some(attachment) = &attachment {
            format!("attachment://{}", attachment.filename)
        } else {
            user.avatar_url()
        })
        .add_not_inlined_field(
            "🌀 Habilidades Equipadas",
            skills
                .iter()
                .map(|skill| skill.data(&fighter).name.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
        .add_not_inlined_field(
            "🧠 Habilidades Aprendidas",
            learned_skills
                .iter()
                .map(|skill| format!("`{}`", skill.data(&fighter).name))
                .collect::<Vec<_>>()
                .join(", "),
        )
        .set_current_timestamp();

    ctx.reply(
        Response::from(embed).set_attachments(if let Some(attachment) = attachment {
            vec![attachment]
        } else {
            vec![]
        }),
    )
    .await?;

    Ok(())
}
